---
title: "Install Lime"
description: "Choose the Windows, Apple silicon Mac, or Linux download and open Lime."
translationKey: "start/install"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Choose a download {#download}

Get Lime from the [official releases](https://github.com/eimg/lime/releases). Choose the download for your operating system and processor.

| Computer | Download | Requirements |
| --- | --- | --- |
| Windows x64 | `.exe` | Windows 10 or later, DirectX, Microsoft Visual C++ 2015–2022 x64 runtime |
| Mac with Apple silicon | `.dmg` | An Apple M-series Mac; the download does not support Intel Macs |
| Linux x86_64 | x86_64 `.tar.gz` | glibc 2.35 or newer, Vulkan, X11 or Wayland, and a file chooser portal |
| Linux ARM64 | aarch64 `.tar.gz` | The same Linux libraries and a compatible ARM64 Vulkan driver |

## Open Lime {#steps}

1. **Windows:** download the EXE and run it. There is no installer or automatic file association. Keep the EXE in a folder where you can find it again.
2. **Mac:** open the DMG, drag **Lime** to **Applications**, and open it there. The macOS release is Developer ID signed and notarized by Apple.
3. **Linux:** extract the archive and run `./lime` from its directory. Keep the archive's files together if using its optional desktop launcher. Nothing is installed automatically.

**Expected result:** Lime opens an empty workspace, ready to create or open an image.

## If Lime does not open {#troubleshooting}

Check that the processor matches the download. Windows builds are unsigned and may show an unknown-publisher warning. On Linux, a blank window can indicate an unavailable or incompatible Vulkan driver; a missing Open/Save dialog can indicate a missing portal backend. See the [Linux runtime requirements](https://github.com/eimg/lime/blob/main/linux/README.txt) for package details.

Find **Check for Updates…** in the **Lime** menu on macOS or **Help** on Windows/Linux. The version also appears in the status bar.

## Next steps {#related}

- [Find your way around the workspace](../../start/workspace/)
- [Make your first edit](../../start/first-edit/)
