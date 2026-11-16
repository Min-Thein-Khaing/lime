# PDF regression fixtures

These small fixtures are authored for Lime and contain no external artwork or
font files. `geometry-fonts-transparency-images.pdf` exercises CropBox, 90° page
rotation, Helvetica text, an embedded Type 3 glyph, half-opacity vector artwork,
and a 2×2 RGB image. The malformed/truncated and invalid-password encrypted
fixtures must return errors. The real headless worker script exercises these
fixtures in each release workflow; this does not claim native or visual PDF QA.

The optional **PDF diagnostics** workflow runs worker cancellation/deadline safety
tests and the real headless worker protocol on macOS and Windows. Release
workflows already check their release worker, so this is not an additional
release gate. To run focused diagnostics locally:

```sh
cargo test --locked pdf::worker::tests
cargo build --locked
python3 scripts/check-pdf-worker.py --binary target/debug/lime
```

On Windows, use `target/debug/lime.exe`. These checks do not open an app window.
