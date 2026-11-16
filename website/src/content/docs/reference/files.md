---
title: "Save projects and export images"
description: "Preserve editable artwork in .lime and choose PNG, JPEG, WebP, or PDF for a flattened copy."
translationKey: "reference/files"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Keep an editable master {#project}

1. Choose **File → Save Project As…** and save a **`.lime`** file.
2. Use **File → Save** for later changes to that project.
3. Keep this master when sharing a flattened export. Reopening the exported image cannot restore its original layer stack or text source.

A Lime project preserves lossless pixels, layers, editable text, paths, guides, masks, blend modes, movement links, and supported retained effects. Personal tool settings, recent colors, and gradient presets belong to your local preferences rather than the project. Older `.imageedit` projects remain readable.

## Export a separate copy {#steps}

1. Choose **File → Export…** (**Cmd/Ctrl+Shift+E**).
2. Choose a format. For JPEG, set quality; for PDF, set DPI and check physical page size.
3. Continue to the destination picker and choose a separate filename.
4. Save the `.lime` master too if there are unsaved edits you want to keep.

**Expected result:** Export writes the visible composition as a flattened copy. It does not change the document's project destination or saved state. The last successfully written format and its relevant options are remembered across restarts; canceled or failed exports do not replace them.

## Choose a format {#formats}

| Format | Best suited to | Keep in mind |
| --- | --- | --- |
| **PNG** | Artwork, screenshots, transparent images | Lossless pixels; preserves transparency |
| **JPEG** | Opaque photographs | Lossy quality setting; no transparency |
| **WebP** | A flattened image that needs transparency | Does not preserve editable layers or text |
| **PDF** | One printable canvas page | Flattened RGB over white; no searchable text or vector paths |

Raster **Save/Save As…** is available only for a single plain layer without paths, guides, or editable text. Use a `.lime` project plus Export when your document has editable structure. PDF is an Export format, never a Save format.

## If saving is blocked {#troubleshooting}

Apply or cancel visible crop, gradient, and transform previews first. Save/Export finish supported inline numeric and text edits before writing. If a text commit fails, correct its retained draft and try again. Confirm the destination is writable and read the exact error before retrying.

## Related guides {#related}

- [Import and export PDFs](../../image/pdf/)
- [Previews and Apply/Cancel](../../basics/previews/)
- [File, text, and recovery problems](../../troubleshooting/files-text-recovery/)
