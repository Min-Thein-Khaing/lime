---
title: "Resize an image or its canvas"
description: "Choose between resampling the whole image, changing canvas bounds, and scaling one layer."
translationKey: "image/resize"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Choose what should change {#choose}

To resize a picture or photo, first decide whether to change its pixels, its canvas space, or just one layer.

| Goal | Command | What changes |
| --- | --- | --- |
| Make the complete image larger or smaller | **Image → Scale Image…** | Resamples the document and its layers |
| Add space around artwork, or trim canvas bounds | **Image → Canvas Size…** | Changes the canvas rectangle without stretching the artwork |
| Resize one raster layer | **Layer → Transform → Scale Layer** | Previews the active layer independently |
| Fit one layer proportionally inside the canvas | **Layer → Fit Layer to Canvas** | Scales the complete layer and centers it |
| Crop/pad a layer to the current canvas | **Layer → Layer to Canvas Size** | Keeps pixel positions while changing that layer's bounds |

## Scale the complete image {#scale}

**Scale Image…** requires every editable text layer to be rasterized, including hidden ones. Save an editable `.lime` original first, then select each text layer and choose **Layer → Rasterize Text Layer**. Alternatively, export a flattened copy and reopen that copy for scaling, keeping the original project editable.

1. Choose **Image → Scale Image…**.
2. Enter the required **Width** or **Height** in pixels. Keep **Lock aspect ratio** enabled to avoid stretching.
3. Choose the resampling method and click **Scale**.
4. Inspect the result at a useful zoom, then save your project or export a copy.

**Nearest** retains hard pixel boundaries. **Bilinear** blends nearby pixels. **Bicubic** provides smoother resampling for general artwork. Enlarging an image creates more pixels but cannot restore detail absent from the original.

## Add canvas space {#canvas}

1. Choose **Image → Canvas Size…**.
2. Enter the new dimensions.
3. Select an anchor to decide where the existing artwork sits in the new rectangle. A centered anchor distributes a size change around it; an edge/corner anchor holds that side in place.
4. Click **Resize** and inspect the canvas edges.

**Expected result:** the document has new bounds while artwork keeps its pixel scale. **Canvas Size…** works with editable text and retains layer pixels beyond the new canvas. Reducing the canvas hides those pixels from the visible composition and export; it does not delete them. Expand the canvas again or use Undo to reveal them.

## Fit a layer without changing the canvas {#fit}

Select a visible, unlocked raster layer and choose **Layer → Fit Layer to Canvas**. It includes transparent margins and may enlarge a small layer. While **Scale Layer** is active, Fit updates that tool's preview and still needs **Apply**. From other tools it commits immediately. Editable text must be rasterized before pixel scaling.

## Related guides {#related}

- [Layer transforms](../../reference/transforms/)
- [Crop an image](../../image/crop/)
- [Import images as layers](../../image/import-layers/)
