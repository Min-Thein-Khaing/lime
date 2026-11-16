---
title: "Combine and position images"
description: "Import images as layers, resize them, and keep related layers moving together."
translationKey: guides/combine-images
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Build the composition {#steps}

1. Open the image that sets the canvas size. Choose **File → Open as Layers…** to add other images to the same document. Imported images are centered; the canvas does not grow to fit them.
2. Select a layer in **Layers**. Use **Layer → Raise Layer** or **Lower Layer**, or drag its row, to change which image appears in front.
3. Choose **Scale Layer** (`Shift+S`). Drag a handle, or type Width and Height with the aspect ratio locked. Choose **Apply** when the preview fits.
4. Choose **Move** (`M`) and drag the layer into position. For precision, edit X/Y and finish with Return, or nudge with arrow keys: one image pixel, or ten with Shift.
5. Use Move's alignment buttons or **Layer → Align** to align the active layer to the canvas. Adjust **Layer → Layer Opacity…** to blend images visually.
6. Save as **`.lime`**, then use **File → Export…** for the flattened result.

## Expected result {#result}

Each imported image remains independently editable in the project. Only the canvas area appears in the export; pixels outside it are not visible in that copy.

## Move related layers together {#links}

Enable the chain toggle on each layer that should move as a group. Moving an enabled member translates every enabled member, including hidden ones. Scaling and rotation remain individual. A position lock on any participating layer prevents the whole move.

Drop Shadow and Stroke layers follow their source automatically through attachments. They do not regenerate after source pixels or text change. Use **Layer → Detach from Source** to move one independently, or **Attach to Layer Above** to attach a raster layer to the layer immediately above it without moving its pixels.

## Troubleshooting {#troubleshooting}

A large imported layer may have handles outside the visible workspace. **Layer → Fit Layer to Canvas** scales the complete layer, including transparent margins, and centers it. In Scale Layer it updates the preview; from other tools it applies immediately. If more space is needed instead, use **Image → Canvas Size…**.

## Related guides {#related}

- [Import images as layers](../../image/import-layers/)
- [Layer transforms](../../reference/transforms/)
- [A layer will not move](../../troubleshooting/moving/)
