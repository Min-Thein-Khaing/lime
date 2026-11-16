---
title: "Selection tools"
description: "Compare geometric, freehand, and connected-color selections."
translationKey: reference/selection
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

Selections limit painting, fills, clearing, and many effects. A selection belongs to the canvas, not to the active layer. Switching layers does not move it.

## Choose a tool {#tools}

| Tool | Key | Gesture and options |
| --- | --- | --- |
| Rectangle Select | R | Drag a rectangle; change corner radius for rounded corners. |
| Oval Select | O | Drag an oval; choose **1:1** for a circle. |
| Free Select | F | Click vertices or drag freehand segments. Click the start or press Return to close; Escape cancels. |
| Magic Wand | W | Click a connected matching region. Adjust Tolerance, Gap radius, and Sample All Layers. |

Shift adds a new region, Control subtracts, and Shift+Control intersects. Control remains Control on macOS for these selection gestures.

## Shape and edge controls {#edges}

Rectangle and Oval share an aspect-ratio preference. Free releases the constraint; fixed ratios also refit the current editable shape around its center. X/Y and Width/Height edit its geometry. Drag inside to move, or use handles to resize. Empty input or Escape restores a numeric edit.

**Antialias edges** softens curved or angled edges without a wide fade. For the wand it affects the completed contour, not the matching threshold. Wand and Free Select apply the preference to new selections. Rectangle/Oval can update a current editable curve.

**Feather new selections** adds a wider soft transition to a newly created region. Its radius is 0–128 image pixels, initially 8, and it starts off. Moving/resizing an existing shape does not feather it again. For a selection already made, use **Select → Feather…**. **Grow…** expands coverage and **Shrink…** contracts it.

## Connected color controls {#wand}

Tolerance 0 matches exactly; higher values admit more variation. Fully transparent pixels compare equally regardless of hidden RGB. Gap radius 0 is off; 1–8 can close small boundary breaks, but large values may seal narrow passages. Sample All Layers finds the region in the visible composite; otherwise it reads the active layer.

**Select → Alpha to Selection** uses the active layer's alpha, including an enabled mask, and also works on text. **Select None** removes the selection restriction. Selection changes can be undone without making the saved artwork dirty.

## Related guides {#related}

- [Selections and soft edges](../../basics/selections/)
- [Remove a background](../../guides/remove-background/)
- [Draw and use paths](../../guides/paths/)
