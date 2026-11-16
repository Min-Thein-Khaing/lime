---
title: "Make your first edit"
description: "Open an image, crop it, save an editable project, and export a PNG copy."
translationKey: "start/first-edit"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Open and crop {#steps}

1. Choose **File → Open…** and select a PNG, JPEG, or WebP image. To start with an empty image instead, choose **File → New Image…**.
2. Select **Crop** from the toolbox or press **C** while the canvas has focus.
3. Drag a box over the area to keep. Drag inside the box to reposition it, or drag a square handle to resize it. Choose **Free** for any proportions or **1:1** for a square.
4. Choose **Image → Apply Crop** or press **Return** on the canvas. Pixels outside the crop are removed from the document; **Edit → Undo** reverses the edit.

**Expected result:** the canvas contains the chosen area at its original pixel detail. Crop changes the document bounds; zoom only changes how large it appears on screen.

## Save an editable copy {#save}

1. Choose **File → Save Project As…**.
2. Choose a name ending in **`.lime`** and a destination you can find again.
3. Save before experimenting with additional edits. A `.lime` project preserves layers and editable content.

## Export a shareable image {#export}

1. Choose **File → Export…** or press **Cmd/Ctrl+Shift+E**.
2. Choose **PNG** and continue to the destination picker.
3. Save to a separate filename. Export creates a flattened copy without changing which project is open or marking unsaved edits as saved.

PNG keeps transparency. JPEG is suitable for opaque photographs; see the format reference before choosing it for transparent artwork.

## If a command is unavailable {#troubleshooting}

Finish any pointer drag. Apply or cancel a visible crop, gradient, or transform preview. Return inside a numeric field finishes that field; Return on the canvas applies the tool preview. If you are editing text, Escape commits the text rather than discarding it.

## Next steps {#related}

- [Layers and transparency](../../basics/layers/)
- [Crop an image](../../image/crop/)
- [Save projects and export images](../../reference/files/)
