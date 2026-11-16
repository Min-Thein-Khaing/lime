---
title: "Make simple color corrections"
description: "Brighten a photo, adjust contrast and saturation, or make a black-and-white image."
translationKey: guides/color-corrections
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Choose a starting point {#choose}

Select the photo's raster layer in **Layers**. Adjustments change the active layer, not every visible layer. To change the whole layer, use **Select → Select None** first; keep a selection when only one area needs correction.

| What you want to change | Start with |
| --- | --- |
| A photo is too dark or too bright | **Colors → Brightness & Contrast…** |
| Colors look too strong or too dull | **Colors → Hue & Saturation…** |
| Remove color from a photo | **Colors → Desaturate…** |
| Adjust the darkest, middle, and brightest tones separately | **Colors → Levels…** |
| Reduce an unwanted color cast | **Colors → Color Balance…** |

Save a `.lime` project before experimenting. You can also use **Layer → Duplicate Layer** and edit the copy, leaving the original beneath it.

## Brighten a dark photo {#brightness}

1. Open **Colors → Brightness & Contrast…**.
2. Raise **Brightness** a little at a time. Watch both the subject and bright areas; too much can wash out detail.
3. Adjust **Contrast** gently if the result looks flat. More contrast increases the difference between light and dark; less reduces it.
4. Compare **Before** and **After**, then **Apply** when the change helps. Use **Cancel** to leave the layer as it was.

**Expected result:** the photo is easier to read while important highlights and shadows still show detail. An adjustment cannot recover detail that was never recorded in the original.

## Adjust color strength or remove color {#saturation}

Open **Colors → Hue & Saturation…** and lower **Saturation** if colors look excessive, or raise it slightly if they look dull. Leave **Hue** unchanged when you want the same colors with a different strength; Hue shifts the colors themselves.

For a black-and-white photo, choose **Colors → Desaturate…**, compare the preview, and Apply. Other layers keep their colors. This is different from **Image → Mode → Grayscale**, which converts the document rather than just the selected photo.

## Try Levels when brightness is not enough {#levels}

Open **Colors → Levels…** and start with the **Value** channel to adjust the color channels together. The histogram shows how the layer's tones are distributed from dark on the left to light on the right.

- **Input black / Input white** set which source tones become black and white. Bringing them inward can strengthen a faded image, but going too far removes shadow or highlight detail.
- **Gamma** adjusts middle tones. Use the preview to brighten or darken the subject while judging the ends of the tonal range separately.
- **Output black / Output white** limit the final darkest and brightest values. Leave these at their defaults for a first correction.

Make small changes and compare Before/After. **Curves…** offers finer control over particular tones when you need it; it can wait until these simpler adjustments are familiar.

## Revise the result {#revise}

Use **Edit → Undo** to reverse a correction. While the layer retains its effect, **Layer → Edit Layer Effect…** reopens its settings and **Remove Layer Effect** restores the retained source. A layer retains one effect at a time; painting or geometry changes bake its appearance. See [Color adjustments and effects](../../reference/effects/) before relying on later re-editing.

Save the `.lime` master and export a separate image to share.

## Related guides {#related}

- [Color adjustments and effects](../../reference/effects/)
- [Selections and soft edges](../../basics/selections/)
- [Save projects and export images](../../reference/files/)
