---
title: "Layers and transparency"
description: "Understand the active layer, stacking, opacity, locks, movement links, and shadow attachments."
translationKey: "basics/layers"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Choose the layer you edit {#active-layer}

A document is a stack of layers. Transparent parts of an upper layer reveal the layers beneath it. The checkerboard shows areas where the combined image is transparent.

Click a row in **Layers** to make it active. Painting, fills, and most effects affect that layer, not every visible layer. Use **File → Open as Layers…** to add another image to the same document.

## Control the appearance {#appearance}

1. Toggle a row's visibility to compare the image with and without it.
2. Choose **Layer → Layer Opacity…** to make the whole layer more transparent.
3. Use **Layer → Raise Layer** or **Lower Layer** to change stacking. Higher layers cover lower layers where their pixels are opaque.
4. Choose a blend mode in the Layers panel when you want overlapping colors to interact. **Normal** uses ordinary layering; **Multiply**, **Screen**, and **Overlay** combine colors differently.

**Expected result:** the combined canvas changes while each layer remains separate. Save as `.lime` to retain the stack. Export creates a flattened image.

## Locks and moving together {#locks-links}

- **Alpha Lock** preserves the existing transparency of a raster layer. You can recolor painted pixels; erasing cannot remove them. It is unavailable for editable text.
- **Position Lock**, shown as a padlock in each row, blocks manual movement and layer transforms. Document-wide image operations still affect locked layers.
- **Movement Link**, the chain toggle, puts enabled layers in one movement set. Moving any member also moves the other enabled members, including hidden layers. A position lock in the set blocks the whole move.

New shadow and outline layers follow their source through a directional attachment. Moving the source moves its followers; moving a shadow directly changes its offset. Choose **Layer → Detach from Source** to make it independent. This attachment does not regenerate the shadow after source edits.

## Related guides {#related}

- [Combine and position images](../../guides/combine-images/)
- [Layer masks](../../basics/masks/)
- [A layer cannot move or transform](../../troubleshooting/moving/)
