---
title: "Selections and soft edges"
description: "Limit edits to part of an image and understand antialiasing, feathering, and selection combinations."
translationKey: "basics/selections"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## What a selection changes {#meaning}

A selection limits where painting, fills, and many effects can change pixels. It does not choose the active layer. A soft selection can give a pixel partial coverage, so an edit fades near the boundary.

The selection outline marks a boundary; it does not show every fractional edge pixel. Zoom in to judge the result. Selection-only changes can be undone without marking an otherwise saved project as modified.

## Make and combine selections {#steps}

1. Choose **Rectangle Select**, **Oval Select**, **Free Select**, or **Magic Wand** under **Select**, or use the toolbox.
2. Drag a rectangle/oval, close a free outline, or click a connected region with the wand.
3. Hold **Shift** to add, **Control** to subtract, or **Shift+Control** to intersect when making another selection. These gesture modifiers use Control on macOS too.
4. Paint or fill on the active layer. Choose **Select → Select None** to remove the limit before editing elsewhere.

Choose **1:1** with Oval Select for a circle. Rectangle and Oval share an aspect ratio; Crop remembers a separate ratio. Choosing a fixed ratio also refits the current editable shape around its center.

## Antialias and feather {#soft-edges}

**Antialias edges** smooths curves and slanted edges over a small fringe. **Feather new selections** spreads coverage over a broader radius. These are different controls and can be combined.

For a selection that already exists, **Select → Feather…** changes its coverage. **Grow…** expands it and **Shrink…** contracts it while preserving soft coverage. Changing the new-selection feather preference alone does not change the current selection.

After combining shapes or editing their coverage, a selection may become a mask without the original rectangle/oval controls. A selection mask and a layer mask serve different purposes: the former limits editing; the latter controls a layer's visibility.

## Related guides {#related}

- [Selection tools](../../reference/selection/)
- [Remove a background](../../guides/remove-background/)
- [Layer masks](../../basics/masks/)
