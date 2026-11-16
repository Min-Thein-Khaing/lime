---
title: "A layer cannot move or transform"
description: "Resolve locks, linked-layer restrictions, offscreen handles, and unfinished previews."
translationKey: troubleshooting/moving
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Check the layer and locks {#locks}

1. Select the intended layer and make it visible. Choose **Move** (`M`) to reposition it; a selection tool moves its selection instead.
2. Check the row's position padlock, or **Layer → Toggle Position Lock**. Position lock blocks pointer moves, X/Y edits, nudges, alignment, Scale/Rotate/Flip/Perspective, Fit Layer to Canvas, and layer content crop.
3. If movement is linked, inspect every enabled chain member and attached descendant. One participating locked layer blocks the entire move, including a hidden member. Lime names a locked participant when a pointer/nudge attempt is blocked.
4. For Scale Layer and other pixel transforms on editable text, keep a `.lime` copy, then use **Layer → Rasterize Text Layer**. Moving editable text does not itself require rasterization.

Move uses a four-arrow cursor without a crosshair and always targets the active layer, even when the pointer is over another layer or empty canvas. Select the intended row in Layers before dragging. Guide grabs still use their own cursors.

## Finish an existing edit {#preview}

Apply or cancel changed Crop/Gradient/transform previews before using commands that are unavailable during a draft. Finish an active pointer drag before changing row metadata or folding the right sidebar.

Inside a numeric field, Return finishes the field; it does not apply an entire transform. Click **Apply** or press Return with the canvas focused. Empty or invalid input and Escape restore a typed edit. Reselecting the active transform preserves its preview; switching tools/layers/tabs cancels it.

## Find the layer or handles {#geometry}

Use **View → Toggle Layer Boundary** to see the active layer's rectangle. A large layer may extend beyond the canvas. In Scale Layer, **Fit Layer to Canvas** previews a proportional fit and centers the whole raster, transparent margins included. From another tool it applies immediately.

For exact movement, type signed X/Y in Move and finish the field. Arrow keys move one image pixel, or ten with Shift. Numeric placement and nudges bypass snapping; turn off Snap to Grid/Guides, or hold Control during a pointer drag, if snapping is unexpected.

## Separate movement relationships {#attachments}

Chain toggles define one group for the document. Turn off a member's toggle to remove it. Attached shadows/outlines follow their source even with chain toggles off. **Layer → Detach from Source** removes that relationship. Moving an attached effect directly adjusts its relative offset; it does not move the source upward through the attachment.

Image-wide transforms still affect locked layers. Use the Image menu only when the whole document should change.

## Related guides {#related}

- [Combine and position images](../../guides/combine-images/)
- [Layer transforms](../../reference/transforms/)
- [Previews and undo](../../basics/previews/)
