---
title: "Start with a blank image"
description: "Create a canvas, add a background and caption, and keep each part editable."
translationKey: start/new-image
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Choose the canvas {#canvas}

Use a blank image for a drawing, caption card, or design assembled from other pictures. For example, make a simple 1,200 × 800 pixel card with a colored background and a title.

1. Choose **File → New Image…**.
2. Enter **Width (px)** as 1200 and **Height (px)** as 800, or use the dimensions required for your image. These are the exported image's pixel dimensions, not its size on screen.
3. Under **Canvas background**, choose **Transparent**, then **Create**. The checkerboard means no painted background is present; it will not appear in the exported image.

Alternatively, **Background color** fills the new image with the current background swatch. See [Choose and reuse colors](../../basics/colors/) to set that swatch before creating the image.

## Add a separate background {#background}

1. The transparent image starts with one layer. Choose **Layer → Rename Layer…** and call it **Drawing** to reserve it for artwork.
2. Choose a foreground color for the card's background.
3. Choose **Layer → New Layer…**, name it **Background**, select **Foreground** under Fill, and choose **Create**.
4. Choose **Layer → Lower Layer** to put Background below Drawing. You can also drag rows to change their order.

The fill choices use the swatches as they are when the layer is created. Changing a swatch later does not recolor an existing background. A **Transparent** new layer is useful for painting without covering the layers beneath it.

## Add artwork and a caption {#artwork}

1. To draw, select **Drawing**, choose a foreground color that contrasts with the background, and paint with Brush or Pencil.
2. To add a picture instead, use **File → Open as Layers…**. [Scale and position it](../../guides/combine-images/) to suit the card.
3. Choose **Text** (`T`), click an empty area, and type a title. Set its font, size, and color in Tool Options, then press **Cmd/Ctrl+Enter** to finish. New text appears above all layers.
4. Use **Move** to position the text. Select the intended row in Layers before dragging.

**Expected result:** the background, artwork, and text are separate layers. You can change the title or move a picture without repainting the background.

## Save and share {#save}

Choose **File → Save Project As…** and save a **`.lime`** master. Use **File → Export…** for a PNG copy to share. PNG preserves any transparent areas; keep an opaque background layer visible if the card should have a solid background.

## Related guides {#related}

- [Choose and reuse colors](../../basics/colors/)
- [Layers and transparency](../../basics/layers/)
- [Add text and effects](../../guides/text-effects/)
