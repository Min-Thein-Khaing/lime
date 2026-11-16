---
title: "Combine and position images"
description: "Import images as layers, resize them, and keep related layers moving together."
translationKey: guides/combine-images
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
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

## Copy part of another image {#clipboard}

1. Open the source image and select the layer containing the pixels you want. **Copy** reads that layer, including its enabled mask, rather than copying all visible layers together.
2. Make a rectangle, oval, or free selection around the part to copy. Moving the selection outline only changes the editing boundary; it does not move the image pixels.
3. Choose **Edit → Copy** (**Cmd/Ctrl+C**). Use **Edit → Cut** (**Cmd/Ctrl+X**) instead only if you want to clear those pixels from the source. Cut needs a visible raster layer with Alpha Lock off.
4. After copying finishes, click the destination document's tab and choose **Edit → Paste** (**Cmd/Ctrl+V**). Finish any text editing first; Paste inside a text draft inserts text.
5. The copied image becomes a new, centered layer without resizing the canvas. Choose **Move** to position it, or **Scale Layer** to resize it and Apply.

**Expected result:** the copied part is independent of the original. Pixels outside an oval/free selection remain transparent in the pasted layer; feathered edges retain their softness. Save both projects if you used Cut and want to keep the source change too.

Use **File → New from Clipboard** if the copied part should become a separate document instead. To move pixels within the same document, Copy and Paste there, then move the pasted layer.

## Line things up with guides and the grid {#guides-grid}

For example, place a vertical guide where the left edges of a picture and caption should line up.

1. Choose **View → Toggle Rulers and Guides**, then select **Move** (`M`).
2. Drag from the left ruler onto the canvas for a vertical guide, or from the top ruler for a horizontal guide. Drag a guide to reposition it. To remove it, drag it out of the canvas workspace and release on a ruler or beyond the workspace.
3. Enable **View → Snap to Guides**. Select the picture's layer and drag it near the guide; its edges or center can snap into alignment. Repeat for the caption layer.
4. For regular spacing, choose **View → Toggle Grid** and **Snap to Grid**. In Move's **Snapping** section, set **Grid spacing (px)** to the interval you want.

Hold **Control** while dragging to bypass snapping and guide grabs, including on macOS. Typed X/Y positions and arrow-key nudges bypass snapping too. Snapping can remain enabled when the grid or guides are hidden; switch off the relevant Snap command if movement still catches unexpectedly.

Guides and the grid help placement and do not appear in exported artwork. Guides are saved in the `.lime` project; grid spacing and snapping choices are personal settings. Use **Layer → Align** for direct alignment to the canvas rather than to a guide.

## Move related layers together {#links}

Enable the chain toggle on each layer that should move as a group. Moving an enabled member translates every enabled member, including hidden ones. Scaling and rotation remain individual. A position lock on any participating layer prevents the whole move.

Drop Shadow and Stroke layers follow their source automatically through attachments. They do not regenerate after source pixels or text change. Use **Layer → Detach from Source** to move one independently, or **Attach to Layer Above** to attach a raster layer to the layer immediately above it without moving its pixels.

## Troubleshooting {#troubleshooting}

A large imported layer may have handles outside the visible workspace. **Layer → Fit Layer to Canvas** scales the complete layer, including transparent margins, and centers it. In Scale Layer it updates the preview; from other tools it applies immediately. If more space is needed instead, use **Image → Canvas Size…**.

## Related guides {#related}

- [Import images as layers](../../image/import-layers/)
- [Layer transforms](../../reference/transforms/)
- [A layer will not move](../../troubleshooting/moving/)
