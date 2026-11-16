---
title: "Draw and use paths"
description: "Edit Bézier anchors, convert paths to selections, and stroke a path."
translationKey: guides/paths
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Draw a path {#steps}

1. Choose **Path** (`P`). Click to place corner anchors, or drag as you place an anchor to pull curve handles.
2. To close an outline, click the first anchor when it is selected alone, or press Return. Escape ends the current path; a later click starts another.
3. Drag an anchor or handle to reshape the curve. Alt-click a visible curve to insert an anchor without changing its shape, then drag to reposition it if needed.
4. Shift-click anchors to select a group. Drag one selected anchor to move the group, or use **Corner**, **Smooth**, and Delete/Backspace on the group. **All Anchors** and **Deselect** are in Tool Options.
5. Use **Paths**, beside Layers, to select a path and toggle its visibility.

## Make a selection or paint the curve {#use}

**Select → Visible Paths to Selection** combines visible closed paths into a selection. Open paths cannot enclose a selection. **Select → Selection to Path** creates closed paths from the current selection.

To paint a path, select a visible path and a visible, paintable raster layer. **Edit → Stroke Path…** opens Pencil/Brush settings such as size, opacity, hardness, flow, and buildup. Choose **Stroke** to paint with the foreground color. The **Paths** panel's **Stroke** button applies the shared settings immediately without opening that dialog.

## Expected result {#result}

The project retains the editable path independently of the pixels it strokes. Editing a path afterward does not redraw an earlier stroke. A path remains a guide until it is converted or stroked; simply displaying it does not add it to exported artwork.

## Handle behavior {#handles}

**Independent** moves one handle alone. **Aligned** keeps the opposite handle on the same line while preserving its length. **Symmetric** mirrors its direction and length. These settings control future handle drags. **Corner** collapses selected handles; **Smooth** aligns or creates them.

For text along a path, remove hard line breaks first; multiline text is rejected even if it fits a fixed box. Select visible editable text and a visible path, then use **Layer → Text Along Path…**. The preview fits one line with alignment, offsets, and Reverse Direction. Apply makes a new raster layer and hides the original text; the source and path remain available in the project.

## Related guides {#related}

- [Selections and soft edges](../../basics/selections/)
- [Add text and effects](../../guides/text-effects/)
- [Painting tool reference](../../reference/painting/)
