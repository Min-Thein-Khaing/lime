---
title: "Import and export PDFs"
description: "Choose PDF pages and raster resolution, then export the current canvas as a single flattened PDF page."
translationKey: "image/pdf"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Import selected PDF pages {#import}

1. Choose **File → Open…** and select a PDF. Encrypted PDFs are unsupported.
2. Select page thumbnails, use **All** or **None**, or enter page numbers and ranges such as `1, 3-5`. **Previous/Next** browse thumbnail batches; **Selected** brings a chosen page into view.
3. Set a resolution from **1–1200 DPI**. Higher DPI creates more pixels and uses more memory. The dialog shows the resulting canvas dimensions.
4. Choose **White** or **Transparent** background. Transparent preserves unpainted areas and source opacity; it does not remove painted white.
5. Choose layers in one document or separate tabs. For layers, choose the first selected page's canvas or **Fit All Pages**. Fit All Pages uses the largest selected width and height, centering every page without scaling it to a common size.
6. Review size warnings and choose **Import**.

**Expected result:** selected pages become raster layers or separate image tabs. They are unsaved projects, not editable PDF objects. Save as `.lime` to keep working.

**File → Open as Layers…** imports one PDF into the existing canvas without resizing it. If Import is unavailable, read its explanation and correct page selection, DPI, or size limits. Cancel discards the import; an in-progress page operation may finish before cancellation settles.

## Export the current canvas {#export}

1. Finish text and apply/cancel other canvas previews.
2. Choose **File → Export…**, then **PDF**.
3. Set DPI and check the displayed physical page size. DPI changes physical size without resampling the canvas. Large canvases may require a higher minimum DPI to stay within PDF page limits.
4. Continue and choose a separate destination.

The result is **one flattened page** containing the complete current canvas as a lossless RGB image over white. Text is not searchable, paths are not vector output, and transparent areas become white. Multiple document tabs are not combined into a multipage PDF.

## Related guides {#related}

- [Import images as layers](../../image/import-layers/)
- [Save projects and export images](../../reference/files/)
- [File, text, and recovery problems](../../troubleshooting/files-text-recovery/)
