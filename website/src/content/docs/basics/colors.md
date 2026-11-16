---
title: "Choose and reuse colors"
description: "Set foreground and background swatches, sample an image, and reuse recent colors."
translationKey: basics/colors
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Foreground and background {#swatches}

The two swatches at the top of the toolbox hold colors for your tools. **Foreground** supplies paint, Bucket Fill, and path/selection strokes. **Background** is used by options such as a new image's Background color fill and an **FG → BG** gradient.

A background swatch is a stored color, not a layer. Changing it does not repaint the canvas. Eraser removes pixels to transparency rather than painting with that swatch.

## Choose a color {#picker}

1. Click a swatch, or choose **Colors → Foreground Color…** or **Background Color…**.
2. Choose a color on the wheel and adjust **Brightness**. To match a known color exactly, enter six digits in **Hex color**, such as `76B7FF`.
3. Choose **Set Color** to accept it. **Cancel** keeps the previous color.

Press **X** on the workspace or choose **Colors → Swap Foreground and Background** to exchange the swatches. **Colors → Default Colors** restores black foreground and white background without clearing recent colors.

## Sample and reuse {#reuse}

Choose **Eyedropper** (`K`) and click inside the canvas to copy the visible composition's color into the foreground swatch. It samples the combined image, so overlapping layers can affect the result.

Open a color picker and click a chip under **Recent colors**, then choose **Set Color**. Lime remembers up to 12 recent colors across tabs and pickers, including Text, shadow, outline, and gradient-stop colors. Accepted colors and Eyedropper samples join this list; canceled picker changes do not.

Swatches and recent colors survive app restarts and belong to your personal settings. They are separate from project history. Recent colors remember RGB only; set transparency separately where the tool offers it, such as a gradient stop's opacity.

## Related guides {#related}

- [Start with a blank image](../../start/new-image/)
- [Paint and retouch](../../guides/paint-retouch/)
- [Fill with colors and gradients](../../guides/fills-gradients/)
