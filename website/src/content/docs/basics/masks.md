---
title: "Hide pixels with a layer mask"
description: "Create a layer mask from a selection, compare it, invert it, and distinguish Apply from Remove."
translationKey: "basics/masks"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Keep the pixels, change visibility {#meaning}

A layer mask controls how much of a raster layer is visible. Fully covered areas remain visible; zero coverage hides them; partial coverage gives a soft edge. The mask combines with the layer's pixel transparency and opacity.

A selection limits your next edit. A layer mask stays attached to one layer and controls its appearance until changed, removed, or applied. Save as `.lime` to retain it.

## Make a mask from a selection {#steps}

1. Select a visible raster layer in **Layers**. Editable text cannot have a layer mask; keep an editable duplicate before rasterizing if necessary.
2. Make a selection over the area to keep visible. Feather it first if you want a soft boundary.
3. Choose **Layer → Mask → From Selection**. The selection's coverage becomes the layer mask, replacing an existing mask on that layer.
4. Choose **Select → Select None** when you no longer need an editing limit.
5. Toggle **Layer → Mask → Enable / Disable** to compare the masked and original appearance without deleting the mask.

**Expected result:** the selected portion remains visible. Hidden portions reveal lower layers or the transparent checkerboard while their original pixels remain stored.

## Revise or finish a mask {#revise}

| Command under Layer → Mask | Result |
| --- | --- |
| **Mask to Selection** | Copies mask coverage into the selection for further selection edits |
| **Invert** | Swaps visible and hidden coverage |
| **From Selection** | Replaces the mask with the current selection |
| **Apply** | Bakes the enabled mask coverage into pixel transparency and removes the separate mask |
| **Remove** | Removes the mask without erasing the underlying pixels |

**Apply** requires an enabled mask and Alpha Lock turned off.

Lime edits masks through selections and these menu commands. There is no separate brush target for painting directly on a layer mask. To change its outline, use **Mask to Selection**, revise the selection, and choose **From Selection** again.

## Related guides {#related}

- [Selections and soft edges](../../basics/selections/)
- [Remove a background](../../guides/remove-background/)
- [Save projects and export images](../../reference/files/)
