---
title: "Import images as layers"
description: "Add image files or clipboard content to an existing document without opening a separate tab."
translationKey: "image/import-layers"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Add another image to the document {#steps}

1. Select the document tab that should receive the new image.
2. Finish any active drag and apply or cancel a transform preview.
3. Choose **File → Open as Layers…** or press **Cmd/Ctrl+L**.
4. Choose the image files to import. Imported images become layers centered on the existing canvas.
5. Select an imported layer in **Layers**. Use **Move** to position it or **Scale Layer** to resize it, then apply the transform.

**Expected result:** the new images appear in the same document. The canvas keeps its current dimensions. A large imported layer may extend beyond it; this does not mean the import was scaled to fit.

## Fit or reveal the imported image {#fit}

For a proportional fit, choose **Layer → Fit Layer to Canvas**. Transparent margins are included, and a small image may be enlarged. If Scale Layer is active, Fit changes its preview; use **Apply** afterward.

To keep the layer at its original pixel size but reveal more space around it, use **Image → Canvas Size…**. This changes the complete document bounds. **Layer → Layer to Canvas Size** instead crops or pads the layer at its existing canvas position.

## Clipboard and PDFs {#other-sources}

**Edit → Paste** or **Cmd/Ctrl+V** adds clipboard image content as a centered layer. **File → New from Clipboard** creates another document instead. While editing text, Paste inserts text into that text draft rather than adding an image layer.

Open as Layers also accepts one PDF at a time. Choose pages and resolution in the import dialog. PDF pages become raster layers on the existing canvas; the command never grows that canvas. The entire selected page set is imported as one undoable edit, or none is added if import fails or is canceled.

## If the import looks missing {#troubleshooting}

Check visibility and stacking in Layers. Select the imported layer and enable **View → Toggle Layer Boundary** to locate its bounds. An oversized layer can be mostly off-canvas. If importing a PDF, review page selection, canvas-size warnings, and any disabled-Import explanation.

## Related guides {#related}

- [Combine and position images](../../guides/combine-images/)
- [Resize an image or canvas](../../image/resize/)
- [Import and export PDFs](../../image/pdf/)
