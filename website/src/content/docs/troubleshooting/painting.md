---
title: "Painting or filling changes nothing"
description: "Check the active layer, selection, locks, brush settings, and clone source."
translationKey: troubleshooting/painting
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Check the target {#target}

If you can't draw or the brush is not working, check these settings before changing the artwork.

1. Select the intended row in **Layers**. A visible image can belong to a different layer from the one being edited.
2. Check that the active layer is visible and has nonzero opacity. A layer above it may cover your new pixels; briefly toggle upper layers off to inspect.
3. If the layer is editable text, keep a project copy and choose **Layer → Rasterize Text Layer** before using pixel tools. To change the words, choose Text instead.
4. Choose **View → Toggle Layer Boundary**. Painting outside the layer's pixel rectangle does not extend that layer. Use **Layer → Layer to Canvas Size** if it needs a canvas-sized pixel area.

## Check the selection and mask {#selection}

Use **Select → Select None** to remove the selection restriction, then try a small stroke and Undo it. A small or offscreen selection can otherwise block the area under the pointer.

An enabled layer mask can hide newly painted pixels. Compare with **Layer → Mask → Enable / Disable**. This does not delete the mask or source pixels. Painting edits the raster, not the mask; use selection-based mask commands to revise coverage.

## Check alpha and brush settings {#settings}

**Layer → Toggle Alpha Lock** controls whether existing alpha can change. With it enabled, fully transparent pixels cannot receive visible paint, and Eraser/Cut/Clear cannot remove pixels. Turn it off if the intended edit needs to change transparency.

Check Opacity, or Strength for Blur/Sharpen. With Buildup enabled, Flow must also be above zero. Low flow and soft hardness can make one pass subtle. Start with a clearly visible test color. **Tools → Reset Current Tool** restores defaults while preserving artwork and colors.

## Check connected fills and cloning {#sampling}

Bucket's Connected mode and Magic Wand require a click inside the canvas. Tolerance 0 matches exactly. If the boundary is made by other layers, enable Sample All Layers; only the active layer receives the fill. Selection mode fills the selection and ignores matching controls.

Clone Stamp needs an Alt-clicked source. Pick it again after switching tabs/layers or changing geometry or sampling. Its source can also be transparent, producing no visible mark. Turn on Show Source to inspect where it reads.

## Related guides {#related}

- [Painting tool reference](../../reference/painting/)
- [Layer masks](../../basics/masks/)
- [Fill with colors and gradients](../../guides/fills-gradients/)
