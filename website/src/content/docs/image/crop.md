---
title: "Crop an image"
description: "Keep part of the canvas using a free or fixed-ratio crop, exact coordinates, or selection bounds."
translationKey: "image/crop"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Keep a rectangular area {#steps}

Image-wide cropping requires every editable text layer to be rasterized, including hidden text layers. Save an editable `.lime` original first, then select each text layer and choose **Layer → Rasterize Text Layer**. To keep the original project fully editable, export a flattened copy and reopen that copy for cropping instead. **Image → Canvas Size…** remains available with editable text, but changes bounds without discarding off-canvas layer pixels.

1. Choose **Crop** in the toolbox or press **C** on the canvas.
2. In **Tool Options**, choose **Free**, **1:1**, **4:3**, **16:9**, **Image**, or a custom ratio using **Ratio W/H**. **Image** uses the current image's proportions.
3. Drag the area to keep. Drag inside to move it, or use the square handles to resize it. A fixed ratio constrains creation, handle drags, and typed dimensions.
4. For exact bounds, edit **X**, **Y**, **Width**, and **Height**. Coordinates use image pixels. Values update the crop draft immediately; empty input leaves it unchanged.
5. Choose **Image → Apply Crop** or press **Return** with the canvas focused. Press **Escape** on the canvas to cancel the draft.

**Expected result:** the document canvas becomes the crop area. Content beyond the new canvas is removed by the crop operation. Use **Edit → Undo** to recover it, and save an editable `.lime` project when satisfied.

## Ratio changes and outside drags {#geometry}

Choosing a fixed ratio refits the current crop inside its old bounds, centered with pixel rounding. **Free** removes the constraint without changing its current shape. Custom ratios use positive whole-number W/H values.

A crop drag may begin outside the canvas and enter it. The crop remains within image pixels. A drag whose area never overlaps the image leaves the existing crop unchanged.

## Other crop commands {#commands}

- **Image → Crop to Selection** uses the current selection's bounding rectangle; it does not cut a free-shaped transparent outline.
- **Image → Crop to Content** reduces the canvas to its nontransparent content.
- **Layer → Crop to Content** crops only the active layer to its opaque content. It does not resize the canvas and is blocked by position lock.

If you want to hide a background while keeping its pixels available, use a [layer mask](../../basics/masks/) instead of cropping.

## Related guides {#related}

- [Resize an image or canvas](../../image/resize/)
- [Selections and soft edges](../../basics/selections/)
- [Previews and Apply/Cancel](../../basics/previews/)
