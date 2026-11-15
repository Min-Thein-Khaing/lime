#!/bin/sh
set -eu

if [ "$(uname -s)" != Darwin ]; then
    echo "This script builds a macOS disk image." >&2
    exit 1
fi

cd "$(dirname "$0")/.."
existing_app=false
case "${1:-}" in
    '') ;;
    --existing-app) existing_app=true ;;
    *) echo "Usage: $0 [--existing-app]" >&2; exit 1 ;;
esac
[ "$#" -le 1 ] || { echo "Usage: $0 [--existing-app]" >&2; exit 1; }
if [ "${LIME_SIGNED_RELEASE:-0}" = 1 ]; then
    : "${APPLE_SIGNING_IDENTITY:?A Developer ID Application identity is required}"
    : "${APPLE_NOTARY_PROFILE:?A validated notarization profile is required}"
fi
if [ -n "${APPLE_NOTARY_PROFILE:-}" ]; then
    : "${APPLE_SIGNING_IDENTITY:?Notarization requires Developer ID Application signing}"
fi
if [ "$existing_app" = true ]; then
    # Repackage the exact release binary without rebuilding or re-signing it.
    cmp macos/Info.plist dist/Lime.app/Contents/Info.plist
    lipo dist/Lime.app/Contents/MacOS/lime -verify_arch arm64
    codesign --verify --deep --strict dist/Lime.app
    if [ -n "${APPLE_SIGNING_IDENTITY:-}" ]; then
        codesign -dv --verbose=4 dist/Lime.app 2>&1 | grep -q '^Authority=Developer ID Application:' || {
            echo 'Sign the existing app with Developer ID before packaging.' >&2
            exit 1
        }
    fi
else
    ./scripts/bundle-macos.sh
fi
if [ -n "${APPLE_NOTARY_PROFILE:-}" ]; then
    if [ "$existing_app" = true ] && xcrun stapler validate dist/Lime.app; then
        spctl --assess --type execute --verbose=2 dist/Lime.app
    else
        ./scripts/notarize-macos.sh dist/Lime.app
    fi
fi

version=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' macos/Info.plist)
app="dist/Lime.app"
dmg="dist/Lime-$version.dmg"
staging=$(mktemp -d "${TMPDIR:-/tmp}/lime-dmg.XXXXXX")
mountpoint="$staging/mount"
candidate="dist/.Lime-$version-$$.dmg"
mounted=false
cleanup() {
    if [ "$mounted" = true ]; then
        if ! hdiutil detach "$mountpoint"; then
            echo "Could not detach $mountpoint; leaving temporary files in $staging." >&2
            return 1
        fi
    fi
    rm -rf "$staging"
    rm -f "$candidate"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

mkdir -p "$staging/content/.background" "$mountpoint" target/dmg-module-cache
xcrun swiftc -module-cache-path target/dmg-module-cache scripts/dmg-background.swift -o "$staging/dmg-background"
"$staging/dmg-background" "$staging/content/.background"
rm "$staging/content/.background/background-preview.png"
ditto "$app" "$staging/content/Lime.app"
ln -s /Applications "$staging/content/Applications"

hdiutil create -volname "Lime $version" -srcfolder "$staging/content" -fs HFS+ -format UDRW "$staging/writable.dmg"
hdiutil attach "$staging/writable.dmg" -mountpoint "$mountpoint" -nobrowse -noautoopen
mounted=true
SetFile -a V "$mountpoint/.background"
osascript scripts/dmg-layout.applescript "$mountpoint"
cp macos/AppIcon.icns "$mountpoint/.VolumeIcon.icns"
SetFile -a V "$mountpoint/.VolumeIcon.icns"
SetFile -a C "$mountpoint"
cmp macos/AppIcon.icns "$mountpoint/.VolumeIcon.icns"
test -s "$mountpoint/.DS_Store"
test "$(readlink "$mountpoint/Applications")" = /Applications
codesign --verify --strict "$mountpoint/Lime.app"
sync
hdiutil detach "$mountpoint"
mounted=false

# Publish only after conversion and verification succeed, preserving the last DMG on failure.
hdiutil convert "$staging/writable.dmg" -format UDZO -imagekey zlib-level=9 -o "$candidate"
hdiutil verify "$candidate"
hdiutil attach "$candidate" -readonly -mountpoint "$mountpoint" -nobrowse -noautoopen
mounted=true
test -s "$mountpoint/.DS_Store"
test "$(readlink "$mountpoint/Applications")" = /Applications
cmp "$staging/content/.background/background.tiff" "$mountpoint/.background/background.tiff"
cmp macos/AppIcon.icns "$mountpoint/.VolumeIcon.icns"
cmp macos/AppIcon.icns "$mountpoint/Lime.app/Contents/Resources/AppIcon.icns"
cmp "$app/Contents/MacOS/lime" "$mountpoint/Lime.app/Contents/MacOS/lime"
codesign --verify --strict "$mountpoint/Lime.app"
hdiutil detach "$mountpoint"
mounted=false
if [ -n "${APPLE_SIGNING_IDENTITY:-}" ]; then
    ./scripts/sign-macos.sh "$candidate"
fi
if [ -n "${APPLE_NOTARY_PROFILE:-}" ]; then
    ./scripts/notarize-macos.sh "$candidate"
    mv "$candidate.notarization.json" "$dmg.notarization.json"
    mv "$candidate.notarization-log.json" "$dmg.notarization-log.json"
fi
mv -f "$candidate" "$dmg"
printf 'Built %s\n' "$dmg"
