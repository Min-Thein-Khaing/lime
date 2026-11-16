---
title: "Fill with colors and gradients"
description: "Fill a connected area or a selection and preview a custom gradient."
translationKey: guides/fills-gradients
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Fill a region {#steps}

1. Select the visible raster layer to edit, then set the foreground color.
2. Choose **Bucket Fill** (`B`). Select **Connected** to fill a matching connected area, or **Selection** to cover the whole current selection.
3. In Connected mode, start with **Tolerance** at 0 for an exact match. Raise it if near colors should join. A small **Gap radius** closes breaks in boundaries without painting over the boundary pixels.
4. Enable **Sample All Layers** when visible layers together define the boundary. The fill still edits only the active layer.
5. Click inside the canvas. Use Undo if the region is too broad and adjust before trying again.

## Expected result {#result}

The connected region or selected area receives the foreground color, limited by the active layer and selection. Selection mode ignores matching and sampling controls. Bucket Fill and Magic Wand remember their own tolerance and gap settings independently.

## Preview a gradient {#gradient}

1. Choose **Gradient** (`G`) and drag a line on the canvas.
2. Choose **FG → BG**, **FG → Clear**, or **Custom**. Foreground/background modes follow the current swatches; Custom uses its own RGBA stops.
3. Adjust the endpoints and choose the linear or radial shape. **Reverse** reverses the colors without moving the geometry.
4. In Custom, select a stop to change RGB, opacity, or position. **Add Stop** splits the largest gap; up to eight stops are supported. The first and last stay at 0% and 100%.
5. Choose **Apply**, or press Return on the canvas. **Cancel** or Escape discards the preview.

## Save a gradient preset {#presets}

Use **Save…** to store the current literal colors, shape, and direction. Expand **Saved gradients** to load, rename, or delete presets. Loading does not move endpoints or change the foreground/background swatches. Up to 32 personal presets survive app restarts; they are separate from project undo history.

If the library reports a write error, keep the current draft and use **Reload Library** before further library changes. A failed write does not replace the previous saved library.

## Related guides {#related}

- [Selections and soft edges](../../basics/selections/)
- [Painting changes nothing](../../troubleshooting/painting/)
- [Previews and undo](../../basics/previews/)
