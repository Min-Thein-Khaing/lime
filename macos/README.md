# macOS distribution signing

Development bundles remain ad hoc signed by default. Distribution requires a
Developer ID Application identity in Keychain and a validated `notarytool`
credential profile. Keep private keys and passwords outside the repository.
The same Apple team and saved profile used for another app can sign Lime;
Lime retains its own `com.eimg.lime` bundle identifier.

Build, sign with hardened runtime and a secure timestamp, notarize/staple the app,
then create, sign, notarize/staple, and verify the DMG:

```sh
export APPLE_SIGNING_IDENTITY='Developer ID Application: YOUR NAME (TEAMID)'
export APPLE_NOTARY_PROFILE='YOUR_SAVED_PROFILE'
LIME_SIGNED_RELEASE=1 ./scripts/package-macos.sh
```

`LIME_SIGNED_RELEASE=1` rejects missing signing/notarization configuration before
building. Optional `APPLE_SIGNING_KEYCHAIN` and `APPLE_NOTARY_KEYCHAIN` select
explicit keychains, including temporary CI keychains. No credential is exported.
Lime contains one executable and needs no extra hardened-runtime entitlements.
If nested executable code is added, sign it before sealing the app bundle.

To sign an existing candidate without rebuilding it:

```sh
./scripts/sign-macos.sh dist/Lime.app
./scripts/notarize-macos.sh dist/Lime.app
LIME_SIGNED_RELEASE=1 ./scripts/package-macos.sh --existing-app
```

The existing-app path checks the candidate's Info.plist, arm64 architecture, and
signature. It preserves the executable; an already stapled/accepted app is not
submitted again. The final DMG replaces the previous local DMG only after every
required check succeeds. Notarization JSON responses and Apple logs are saved
beside their artifacts. A failed submission, non-Accepted status, failed ticket
validation, or Gatekeeper rejection stops packaging. Failed DMG submissions keep
their logs under the temporary candidate name in `dist/`.

Check the signed executable's PDF worker with:

```sh
python3 scripts/check-pdf-worker.py --binary dist/Lime.app/Contents/MacOS/lime
python3 scripts/test-macos-signing.py
```

Also launch the exact stapled app to check GPUI/Metal rendering. On a downloaded
copy with quarantine metadata, verify normal Finder launch. A local app without
quarantine is insufficient to check the download experience.

If Apple returns HTTP 403 with a missing/expired agreement, the Apple team's
Account Holder must resolve the pending agreement; do not replace credentials
or weaken Gatekeeper to work around it. Reusing a profile does not require
reading its password or private key.

Publishing is separate from packaging. Update checksums whenever signed bytes
change. Replace an existing published asset only when explicitly requested by
the owner, preserving the other assets and recording the packaging change in
the release notes. Never silently fall back to an ad hoc release signature.
