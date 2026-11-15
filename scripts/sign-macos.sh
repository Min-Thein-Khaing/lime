#!/bin/sh
# Sign completed release artifacts; never fall back to an ad hoc signature.
set -eu

[ "$(uname -s)" = Darwin ] || { echo 'macOS is required.' >&2; exit 1; }
[ "$#" -eq 1 ] || { echo "Usage: $0 <app-or-dmg>" >&2; exit 1; }
: "${APPLE_SIGNING_IDENTITY:?Set a Developer ID Application signing identity}"
artifact=$1
case "$artifact" in
    *.app) [ -d "$artifact" ]; set -- --options runtime ;;
    *.dmg) [ -f "$artifact" ]; set -- ;;
    *) echo 'Expected an app bundle or DMG.' >&2; exit 1 ;;
esac
if [ -n "${APPLE_SIGNING_KEYCHAIN:-}" ]; then
    set -- "$@" --keychain "$APPLE_SIGNING_KEYCHAIN"
fi
codesign --force --timestamp --sign "$APPLE_SIGNING_IDENTITY" "$@" "$artifact"
codesign --verify --deep --strict --verbose=2 "$artifact"
# An ordinary development identity cannot be used for direct distribution.
codesign -dv --verbose=4 "$artifact" 2>&1 | grep -q '^Authority=Developer ID Application:' || {
    echo 'Release signing requires a Developer ID Application certificate.' >&2
    exit 1
}
