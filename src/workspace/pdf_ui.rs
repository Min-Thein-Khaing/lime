use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::{StreamExt, channel::mpsc};

use gpui::{Context, Div, Window, div, prelude::*, px, rgb, rgba};

use super::Workspace;
use crate::{
    document::{self, Document, DocumentContent, ImageLayer, InsertStop, LayerId},
    pdf::{
        self, PdfBackground, PdfCanvas, PdfControl, PdfMode, PdfOptions, PdfProgress, PdfSource,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PdfTarget {
    NewDocument,
    Layers {
        tab_id: usize,
        generation: u64,
        above: LayerId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(super) struct PdfPreferences {
    pub dpi: u32,
    pub mode: PdfMode,
    pub background: PdfBackground,
    pub canvas: PdfCanvas,
}

impl Default for PdfPreferences {
    fn default() -> Self {
        Self {
            dpi: pdf::DEFAULT_DPI,
            mode: PdfMode::Layers,
            background: PdfBackground::White,
            canvas: PdfCanvas::FirstPage,
        }
    }
}

impl PdfPreferences {
    fn remember_success(&mut self, options: &PdfOptions, target: PdfTarget) {
        self.dpi = options.dpi;
        self.background = options.background;
        if target == PdfTarget::NewDocument {
            self.mode = options.mode;
            self.canvas = options.canvas;
        }
    }
}

pub(super) enum PendingOpen {
    Pdf { path: PathBuf },
    Raster(Box<Document>),
}

pub(super) struct PdfJob {
    token: u64,
    name: String,
    control: PdfControl,
    progress: PdfProgress,
}

impl PdfJob {
    fn can_publish(&self, token: u64) -> bool {
        self.token == token && !self.control.canceled()
    }
    pub(super) fn label(&self) -> String {
        if self.control.canceled() {
            return "Canceling PDF import…".into();
        }
        match self.progress {
            PdfProgress::Reading => "Reading PDF…".into(),
            PdfProgress::Inspecting => "Inspecting PDF pages…".into(),
            PdfProgress::Rendering { page, done, total } => {
                format!("Rendering page {page} — {done} of {total} complete")
            }
            PdfProgress::Preparing { done, total } => {
                format!("Preparing images — {done} of {total} complete")
            }
        }
    }
}

impl Drop for PdfJob {
    fn drop(&mut self) {
        self.control.cancel();
    }
}

struct LayerDestination {
    state: super::history::State,
    above: LayerId,
}

impl LayerDestination {
    fn new(tab: &super::Tab, above: LayerId) -> Self {
        Self {
            state: tab.state(),
            above,
        }
    }

    fn prepare(
        mut self,
        page_numbers: &[usize],
        pages: Vec<image::RgbaImage>,
        control: &PdfControl,
        progress: impl Fn(PdfProgress),
    ) -> anyhow::Result<super::PreparedChange> {
        control.check()?;
        pdf::validate_page_numbers(page_numbers, pdf::MAX_PDF_PAGES)?;
        anyhow::ensure!(
            page_numbers.len() == pages.len(),
            "The PDF import did not render the complete page selection."
        );
        let total = pages.len() + 1;
        progress(PdfProgress::Preparing { done: 0, total });
        control.check()?;
        let sizes: Vec<_> = pages.iter().map(image::RgbaImage::dimensions).collect();
        anyhow::ensure!(
            !pages.is_empty() && self.state.content.layer(self.above).is_some(),
            "The PDF import target or pages are no longer available."
        );
        self.state.content.room_for_sizes(&sizes).map_err(|_| {
            anyhow::anyhow!("The selected pages no longer fit in the target image.")
        })?;
        let mut above = self.above;
        for (index, pixels) in pages.into_iter().enumerate() {
            control.check()?;
            let inserted = self.state.content.insert_centered_layers(
                above,
                vec![ImageLayer {
                    name: format!("Page {}", page_numbers[index]),
                    pixels,
                }],
            );
            anyhow::ensure!(
                inserted.added == 1,
                "The selected pages no longer fit in the target image."
            );
            above = inserted
                .top
                .ok_or_else(|| anyhow::anyhow!("The PDF import has no pages."))?;
            self.state.active_layer = above;
            progress(PdfProgress::Preparing {
                done: index + 1,
                total,
            });
            control.check()?;
        }
        let patch =
            document::RenderedCanvas::prepare_checked(&self.state.content, &|| control.check())?;
        control.check()?;
        progress(PdfProgress::Preparing { done: total, total });
        control.check()?;
        Ok((
            self.state.content,
            self.state.active_layer,
            self.state.selection,
            patch,
        ))
    }
}

enum PdfResult {
    Documents(Vec<Document>),
    Layers {
        change: Box<super::PreparedChange>,
        added: usize,
    },
}

impl Workspace {
    pub(super) fn schedule_pending_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.open_scheduled
            || self.pending_opens.is_empty()
            || self.dialog_open
            || self.form.is_some()
            || self.creating
            || self.pdf_job.is_some()
            || self.active_tab().is_some_and(super::Tab::busy)
        {
            return;
        }
        self.open_scheduled = true;
        let entity = cx.weak_entity();
        window.on_next_frame(move |window, cx| {
            let _ = entity.update(cx, |this, cx| {
                this.open_scheduled = false;
                if this.dialog_open
                    || this.form.is_some()
                    || this.creating
                    || this.pdf_job.is_some()
                    || this.active_tab().is_some_and(super::Tab::busy)
                {
                    return;
                }
                if let Some(pending) = this.pending_opens.pop_front() {
                    match pending {
                        PendingOpen::Pdf { path } => {
                            this.request_pdf_import(path, PdfTarget::NewDocument, window, cx);
                        }
                        PendingOpen::Raster(document) => this.show_document(*document, window, cx),
                    }
                }
            });
        });
    }

    fn start_pdf_job(
        &mut self,
        name: String,
        initial: PdfProgress,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (u64, PdfControl, mpsc::UnboundedSender<PdfProgress>) {
        self.pdf_import_token = self.pdf_import_token.wrapping_add(1);
        let token = self.pdf_import_token;
        let control = PdfControl::default();
        self.pdf_job = Some(PdfJob {
            token,
            name,
            control: control.clone(),
            progress: initial,
        });
        self.space_pan = false;
        self.focus.focus(window);
        let (sender, mut updates) = mpsc::unbounded();
        cx.spawn_in(window, async move |workspace, cx| {
            while let Some(progress) = updates.next().await {
                if workspace
                    .update_in(cx, |workspace, _, cx| {
                        if let Some(job) =
                            workspace.pdf_job.as_mut().filter(|job| job.token == token)
                        {
                            job.progress = progress;
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
        cx.notify();
        (token, control, sender)
    }

    pub(super) fn cancel_pdf_job(&mut self, cx: &mut Context<Self>) {
        if let Some(job) = &self.pdf_job
            && !job.control.canceled()
        {
            job.control.cancel();
            cx.notify();
        }
    }

    pub(super) fn pdf_progress_overlay(&self, cx: &Context<Self>) -> Option<Div> {
        use super::view::{BORDER, MUTED, PANEL};
        let job = self.pdf_job.as_ref()?;
        let canceled = job.control.canceled();
        Some(
            div()
                .absolute()
                .size_full()
                .occlude()
                .bg(rgba(0x0b0d12bb))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w(px(380.))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(BORDER))
                        .bg(rgb(PANEL))
                        .child(div().text_lg().child("PDF Import"))
                        .child(job.name.clone())
                        .child(job.label())
                        .when(canceled, |panel| {
                            panel.child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(MUTED))
                                    .child("Waiting for the current PDF step to finish."),
                            )
                        })
                        .child(
                            super::dialogs::form_button(
                                if canceled { "Canceling…" } else { "Cancel" },
                                false,
                            )
                            .when(!canceled, |button| {
                                button
                                    .on_click(cx.listener(|this, _, _, cx| this.cancel_pdf_job(cx)))
                            }),
                        ),
                ),
        )
    }

    pub(super) fn request_pdf_import(
        &mut self,
        path: PathBuf,
        target: PdfTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dialog_open || self.form.is_some() || self.creating || self.pdf_job.is_some() {
            self.errors
                .push("Wait for the current file operation to finish.".into());
            cx.notify();
            return;
        }
        let name = document::file_name(&path);
        let (token, control, updates) =
            self.start_pdf_job(name.clone(), PdfProgress::Reading, window, cx);
        let job_path = path.clone();
        let job = cx.background_executor().spawn(async move {
            pdf::read_source(&job_path, &control, |progress| {
                let _ = updates.unbounded_send(progress);
            })
        });
        cx.spawn_in(window, async move |workspace, cx| {
            let result = job.await;
            let _ = workspace.update_in(cx, |workspace, window, cx| {
                if workspace
                    .pdf_job
                    .as_ref()
                    .is_none_or(|job| job.token != token)
                {
                    return;
                }
                let job = workspace.pdf_job.take().unwrap();
                if job.can_publish(token) {
                    match result {
                        Ok(source) => {
                            workspace.pdf_import_form(path, Arc::new(source), target, window, cx);
                        }
                        Err(error) if error.is::<pdf::PdfCanceled>() => {}
                        Err(error) => workspace.errors.push(format!("{name}: {error:#}")),
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(super) fn validate_pdf_target(
        &self,
        target: PdfTarget,
        sizes: &[(u32, u32)],
    ) -> anyhow::Result<()> {
        if let PdfTarget::Layers {
            tab_id,
            generation,
            above,
        } = target
        {
            let tab = self
                .active_tab()
                .filter(|tab| {
                    tab.id == tab_id
                        && tab.history.generation() == generation
                        && !tab.busy()
                        && tab.document.content.layer(above).is_some()
                })
                .ok_or_else(|| {
                    anyhow::anyhow!("The target image changed. Close this dialog and try again.")
                })?;
            tab.document.content.room_for_sizes(sizes).map_err(|stop| match stop {
                InsertStop::Full => anyhow::anyhow!("The selected pages exceed the document's remaining layer capacity. Choose fewer pages."),
                InsertStop::TooLarge => anyhow::anyhow!("The selected pages exceed the document's remaining pixel budget. Lower the DPI or choose fewer pages."),
            })?;
        }
        Ok(())
    }

    pub(super) fn begin_pdf_import(
        &mut self,
        source: Arc<PdfSource>,
        options: PdfOptions,
        target: PdfTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pdf_job.is_some() {
            return;
        }
        let valid = pdf::validate_options(&source.info, &options)
            .and_then(|sizes| self.validate_pdf_target(target, &sizes));
        if let Err(error) = valid {
            self.errors.push(error.to_string());
            cx.notify();
            return;
        }
        self.cancel_move(window, cx);
        self.clear_content_previews(window, cx);
        let destination = match target {
            PdfTarget::Layers { above, .. } => self
                .active_tab()
                .map(|tab| LayerDestination::new(tab, above)),
            _ => None,
        };
        let name = document::file_name(&options.path);
        let (token, control, updates) =
            self.start_pdf_job(name.clone(), PdfProgress::Inspecting, window, cx);
        let job_options = options.clone();
        let job = cx.background_executor().spawn(async move {
            let progress = |progress| {
                let _ = updates.unbounded_send(progress);
            };
            let pages = pdf::render_pages(&source, &job_options, &control, progress)?;
            drop(source);
            control.check()?;
            Ok::<_, anyhow::Error>(if matches!(target, PdfTarget::NewDocument) {
                PdfResult::Documents(prepare_documents(&job_options, pages, &control, progress)?)
            } else {
                let destination = destination
                    .ok_or_else(|| anyhow::anyhow!("The target image is no longer open."))?;
                let added = pages.len();
                PdfResult::Layers {
                    change: Box::new(destination.prepare(
                        &job_options.pages,
                        pages,
                        &control,
                        progress,
                    )?),
                    added,
                }
            })
        });
        cx.spawn_in(window, async move |workspace, cx| {
            let result = job.await;
            let _ = workspace.update_in(cx, |workspace, window, cx| {
                if workspace
                    .pdf_job
                    .as_ref()
                    .is_none_or(|job| job.token != token)
                {
                    return;
                }
                let job = workspace.pdf_job.take().unwrap();
                if !job.can_publish(token) {
                    workspace.notice("PDF import canceled.", cx);
                } else {
                    match result {
                        Ok(PdfResult::Documents(documents)) => {
                            for document in documents {
                                workspace.show_document(document, window, cx);
                            }
                            workspace
                                .preferences
                                .pdf_preferences
                                .remember_success(&options, target);
                        }
                        Ok(PdfResult::Layers { change, added }) => {
                            if let Err(error) = workspace.validate_pdf_target(target, &[]) {
                                workspace.notice(&error.to_string(), cx);
                            } else if let PdfTarget::Layers {
                                tab_id, generation, ..
                            } = target
                                && workspace.publish_change(
                                    tab_id,
                                    generation,
                                    super::Change::Edit,
                                    Some(*change),
                                    window,
                                    cx,
                                )
                            {
                                workspace
                                    .preferences
                                    .pdf_preferences
                                    .remember_success(&options, target);
                                let layers = if added == 1 { "layer" } else { "layers" };
                                workspace
                                    .notice(&format!("Added {added} {layers} from the PDF."), cx);
                            }
                        }
                        Err(error) if error.is::<pdf::PdfCanceled>() => {}
                        Err(error) => workspace.errors.push(format!("{name}: {error:#}")),
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}

fn prepare_documents(
    options: &PdfOptions,
    pages: Vec<image::RgbaImage>,
    control: &PdfControl,
    progress: impl Fn(PdfProgress),
) -> anyhow::Result<Vec<Document>> {
    control.check()?;
    anyhow::ensure!(!pages.is_empty(), "The PDF import has no rendered pages.");
    anyhow::ensure!(
        pages.len() <= document::MAX_LAYERS,
        "The PDF import has too many pages."
    );
    pdf::validate_page_numbers(&options.pages, pdf::MAX_PDF_PAGES)?;
    anyhow::ensure!(
        options.pages.len() == pages.len(),
        "The PDF import did not render the complete page selection."
    );
    for page in &pages {
        Document::validate_size(page.width(), page.height())?;
    }
    let name = document::file_name(&options.path);
    let total = if options.mode == PdfMode::Separate {
        pages.len()
    } else {
        1
    };
    progress(PdfProgress::Preparing { done: 0, total });
    control.check()?;
    let documents = if options.mode == PdfMode::Separate {
        let count = pages.len();
        let stem = Path::new(&name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .filter(|stem| !stem.trim().is_empty())
            .unwrap_or("PDF");
        let mut documents = Vec::with_capacity(count);
        for (index, pixels) in pages.into_iter().enumerate() {
            control.check()?;
            let title = if count == 1 {
                name.clone()
            } else {
                format!("{stem} ({})", options.pages[index])
            };
            let document = Document::from_content_checked(
                title,
                DocumentContent::single_layer(format!("Page {}", options.pages[index]), pixels),
                &|| control.check(),
            )?;
            control.check()?;
            documents.push(document);
            progress(PdfProgress::Preparing {
                done: index + 1,
                total,
            });
        }
        documents
    } else {
        let document =
            Document::from_content_checked(name, document_from_layers(pages, options)?, &|| {
                control.check()
            })?;
        control.check()?;
        progress(PdfProgress::Preparing { done: 1, total });
        vec![document]
    };
    control.check()?;
    Ok(documents)
}

fn document_from_layers(
    pages: Vec<image::RgbaImage>,
    options: &PdfOptions,
) -> anyhow::Result<DocumentContent> {
    pdf::validate_page_numbers(&options.pages, pdf::MAX_PDF_PAGES)?;
    anyhow::ensure!(
        options.pages.len() == pages.len(),
        "The PDF import did not render the complete page selection."
    );
    let sizes: Vec<_> = pages.iter().map(image::RgbaImage::dimensions).collect();
    let canvas = pdf::layered_canvas_size(&sizes, options.canvas)?;
    let mut pages = pages.into_iter();
    let first = pages
        .next()
        .ok_or_else(|| anyhow::anyhow!("The PDF import has no pages."))?;
    let mut content = DocumentContent::single_layer(format!("Page {}", options.pages[0]), first);
    if canvas != content.size() {
        anyhow::ensure!(
            content
                .resize_canvas(canvas.0, canvas.1, crate::transform::Anchor::Center)
                .is_some(),
            "The PDF canvas could not be sized."
        );
    }
    // Canvas resizing and imported layers round odd centers differently. Use
    // the same centering rule for the first page as for every inserted page.
    content.align_layer(content.top_layer_id(), crate::geometry::Alignment::Both);
    content.room_for_sizes(&sizes[1..]).map_err(|_| {
        anyhow::anyhow!("The selected pages exceed the document's layer or pixel budget.")
    })?;
    let above = content.top_layer_id();
    let layers = pages
        .enumerate()
        .map(|(index, pixels)| ImageLayer {
            name: format!("Page {}", options.pages[index + 1]),
            pixels,
        })
        .collect();
    let inserted = content.insert_centered_layers(above, layers);
    anyhow::ensure!(
        inserted.added == sizes.len() - 1,
        "The complete PDF layer stack could not be prepared."
    );
    Ok(content)
}
