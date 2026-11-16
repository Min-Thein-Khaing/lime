---
title: "Painting and retouching tools"
description: "Brush coverage, smoothing, cloning, alpha locks, and tool resets."
translationKey: reference/painting
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Choose the operation {#tools}

| Tool | Behavior |
| --- | --- |
| Pencil | Hard-edged foreground strokes; no smoothing. |
| Brush | Foreground strokes with adjustable hardness and smoothing. |
| Eraser | Removes alpha; does not replace pixels with the background swatch. |
| Blur Brush / Sharpen Brush | Changes local detail using the stroke's starting pixels. |
| Clone Stamp | Copies from an Alt-clicked source; `Shift+C` selects it. |
| Eyedropper | Samples the visible composition into the foreground swatch; key `K`. |

Paint and retouch strokes are clipped to the canvas, active layer, and selection. They can start outside the canvas and enter while held. Eyedropper and clone-source picking require an inside-canvas point.

## Opacity, flow, and smoothing {#settings}

Size is measured in image pixels. `[` and `]` change the size independently of zoom. Hardness controls the edge softness where supported. Opacity limits the change during one stroke; Blur/Sharpen label it Strength.

Buildup starts off. With it enabled, Flow accumulates dab coverage toward the stroke's opacity limit. Lower flow can need several passes within one stroke. Release and begin a new stroke to compound the result. Zero opacity, or zero flow with buildup enabled, changes nothing.

Brush, Eraser, Blur, Sharpen, and Clone Stamp remember independent **Off**, **Light**, or **Strong** smoothing. Smoothing trails the pointer briefly, catches up when it stops, and finishes at the exact endpoint on release. Pencil and path/selection outline strokes stay unsmoothed.

## Clone source controls {#clone}

Alt-click sets the source. Sample All Layers reads a fixed snapshot of the visible composition at stroke start; the destination remains the active visible raster layer. Aligned preserves the offset across strokes; unaligned strokes restart at the selected source. Show Source displays the source position and footprint. Sampling changes clear the source; tab/layer or geometry changes require a fresh source.

## Locks and resetting {#reset}

Alpha Lock preserves existing alpha: paint colors existing pixels, while Eraser/Cut/Clear cannot remove them. Editable text needs **Layer → Rasterize Text Layer** before pixel painting.

**Tools → Reset Current Tool** restores the active tool's defaults. **Reset All Tool Settings…** asks before resetting every tool. These commands preserve artwork, swatches, and saved gradient presets.

## Related guides {#related}

- [Paint and retouch](../../guides/paint-retouch/)
- [Fill with colors and gradients](../../guides/fills-gradients/)
- [Painting changes nothing](../../troubleshooting/painting/)
