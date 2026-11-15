#!/bin/sh
# Reuse a saved notarytool profile without exporting credentials.
set -eu

[ "$(uname -s)" = Darwin ] || { echo 'macOS is required.' >&2; exit 1; }
[ "$#" -eq 1 ] || { echo "Usage: $0 <signed-app-or-dmg>" >&2; exit 1; }
: "${APPLE_NOTARY_PROFILE:?Set a validated notarytool keychain profile}"
artifact=$1
case "$artifact" in
    *.app) [ -d "$artifact" ] ;;
    *.dmg) [ -f "$artifact" ] ;;
    *) echo 'Expected an app bundle or DMG.' >&2; exit 1 ;;
esac
codesign --verify --deep --strict "$artifact"
codesign -dv --verbose=4 "$artifact" 2>&1 | grep -q '^Authority=Developer ID Application:' || {
    echo 'Notarization requires Developer ID Application signing first.' >&2
    exit 1
}
staging=$(mktemp -d "${TMPDIR:-/tmp}/lime-notary.XXXXXX")
trap 'rm -rf "$staging"' EXIT
trap 'exit 1' HUP INT TERM
upload=$artifact
case "$artifact" in
    *.app)
        upload="$staging/Lime.zip"
        ditto -c -k --keepParent "$artifact" "$upload"
        ;;
esac
set -- --keychain-profile "$APPLE_NOTARY_PROFILE"
if [ -n "${APPLE_NOTARY_KEYCHAIN:-}" ]; then
    set -- "$@" --keychain "$APPLE_NOTARY_KEYCHAIN"
fi
# Keep Apple's response/log beside the artifact, including on rejection.
result="$artifact.notarization.json"
submit_status=0
xcrun notarytool submit "$upload" "$@" --wait --timeout 30m --output-format json > "$result" || submit_status=$?
cat "$result"
submission=$(python3 -c '
import json,sys
try:
    print(json.load(open(sys.argv[1])).get("id", ""))
except (ValueError, OSError):
    pass
' "$result")
if [ -n "$submission" ]; then
    xcrun notarytool log "$submission" "$@" "$artifact.notarization-log.json"
fi
[ "$submit_status" -eq 0 ] || exit "$submit_status"
python3 -c 'import json,sys; sys.exit(0 if json.load(open(sys.argv[1])).get("status") == "Accepted" else "Apple did not accept notarization; see the saved log.")' "$result"
for action in staple validate; do
    # CloudKit can briefly lag behind an accepted submission.
    attempt=0
    while :; do
        status=0
        xcrun stapler "$action" "$artifact" || status=$?
        [ "$status" -eq 0 ] && break
        attempt=$((attempt + 1))
        case "$status" in 65|68) ;; *) exit "$status" ;; esac
        [ "$attempt" -lt 4 ] || exit "$status"
        sleep "$((attempt * 5))"
    done
done
codesign --verify --deep --strict "$artifact"
case "$artifact" in
    *.app) spctl --assess --type execute --verbose=2 "$artifact" ;;
    *.dmg) spctl --assess --type open --context context:primary-signature --verbose=2 "$artifact" ;;
esac
