---
title: "Color adjustments and effects"
description: "Preview color changes, re-edit a retained effect, and repeat successful settings."
translationKey: reference/effects
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Choose an effect {#choose}

For help choosing an adjustment and making a first correction, see [Make simple color corrections](../../guides/color-corrections/).

**Colors** contains Brightness & Contrast, Hue & Saturation, Color Balance, Levels, Curves, Desaturate, and Invert. These adjustments preserve the layer's alpha. Levels and Curves offer channel controls; their histogram ignores fully transparent pixels.

**Filters** contains Blur, Sharpen, Drop Shadow, and Stroke. Blur/Sharpen work on the raster layer and follow the selection, mixing both color and alpha unless alpha is locked. Drop Shadow/Stroke create a separate layer beneath a visible raster or editable-text source.

## Preview and apply {#steps}

1. Select the visible layer and make a selection if the adjustment should affect only part of it.
2. Open the command from Colors or Filters. Change its settings while viewing the composed image, including surrounding layers, masks, and opacity.
3. Compare **Before** and **After**. Before only changes the view: Apply still uses the chosen parameters.
4. **Reset** restores the built-in defaults and returns to After. **Apply** commits the chosen effect; **Cancel** leaves the pixels unchanged.

An identity adjustment or zero-strength effect creates no image-history entry. Successful parameters are remembered across tabs and restarts; canceled or failed previews do not replace them.

## Re-edit a layer effect {#retained}

A raster layer can retain one color adjustment or Blur/Sharpen source for later editing. Use **Layer → Edit Layer Effect…** to reopen it, **Remove Layer Effect** to restore its retained source, or **Bake Layer Effect** to keep the appearance and discard re-editable effect data.

Painting or geometry changes bake the retained result; a new effect replaces the previous retained slot using its current appearance. Moving the layer, changing opacity, or directly changing its mask retains the effect. Save `.lime` to preserve retained data.

## Repeat and generated layers {#repeat}

**Filters → Repeat Last Effect** applies the last successful settings to the current eligible layer and selection. **Reopen Last Effect…** opens those settings with a preview. The shortcuts are Cmd/Ctrl+Alt+F and Cmd/Ctrl+Alt+Shift+F.

Drop Shadow and Stroke accept editable text without rasterizing it. Their generated pixels follow source movement but stay frozen after source edits. An enabled mask affects the source silhouette; Stroke ignores source opacity and selection when forming the outside outline. Use [the text/effects guide](../../guides/text-effects/) for that workflow.

## Related guides {#related}

- [Previews and undo](../../basics/previews/)
- [Add text and effects](../../guides/text-effects/)
- [Layers and transparency](../../basics/layers/)
