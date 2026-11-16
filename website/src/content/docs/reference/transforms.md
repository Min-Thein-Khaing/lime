---
title: "Layer transforms"
description: "Scale, rotate, flip, and change perspective without repeatedly resampling a draft."
translationKey: reference/transforms
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

Choose transforms from **Layer → Transform**, **Tools → Transform**, or the toolbox. They preview the active visible raster layer. Rasterize editable text first; a position lock blocks these operations.

## Shared preview behavior {#preview}

**Apply** or Return on the canvas commits the preview as one undo step. **Cancel** or Escape on the canvas discards it. Return inside a numeric field finishes that edit without applying the whole transform. Empty input or Escape restores the numeric edit.

Repeated drags keep the original source for the final commit. Reselecting the active tool preserves the preview. Switching tools, tabs, or layers cancels it. After Apply/Cancel, selecting the tool again starts a new session; Scale Layer also restarts with a canvas click.

The selection stays in canvas coordinates during these layer transforms.

## Scale Layer {#scale}

Key **Shift+S**. Drag a side/corner handle or type Width/Height. Aspect locking starts on; Shift reverses it while dragging. The nine-point **Size anchor** controls typed resizing; handle drags keep the opposite edge/corner fixed.

**Nearest** preserves hard source pixels; **Bilinear** and **Bicubic** interpolate them. The selected method is used for Apply. Preview resolution is bounded, so inspect the applied result at 100% before judging fine detail.

**Fit Layer to Canvas** proportionally scales the complete raster, transparent margins included, and centers it. It may enlarge a small layer. In Scale Layer it adjusts the draft; elsewhere it applies immediately. **Layer to Canvas Size** instead crops/pads without scaling pixels.

## Rotate and Flip {#rotate-flip}

**Rotate** uses Shift+R. Drag or type an angle in degrees. Pick one of nine pivot anchors, edit signed canvas X/Y, or drag the circular pivot crosshair. Shift-drag snaps to absolute 15-degree angles around that pivot.

**Flip** uses Shift+F. Choose Horizontal/Vertical or drag to choose a direction, then Apply.

## Perspective {#perspective}

Key **Shift+P**. Drag the four corner grips, or select one in the corner selector and type canvas X/Y. **Reset Corner** restores that point; **Reset All** restores the original rectangle. Crossing corners or an oversized result disables Apply while leaving the draft editable.

Image-wide quarter-turns and flips are under **Image → Transform** and affect every layer. Use **Image → Scale Image…** to scale the entire document.

## Related guides {#related}

- [Resize an image or canvas](../../image/resize/)
- [Combine images](../../guides/combine-images/)
- [A layer will not move](../../troubleshooting/moving/)
