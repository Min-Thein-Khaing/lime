//! A shared export draft keeps format-specific options out of personal settings
//! until the existing background writer reports success.
use anyhow::{Result, ensure};
use gpui::{Context, Div, KeyDownEvent, Window, div, prelude::*, rgb};

use super::{
    ACCENT, Field, FieldMode, FormKind, MUTED, Workspace, choice_button, pdf_input, segmented,
};
use crate::document::FileFormat;
use crate::workspace::files::{ExportFormat, ExportType};
use crate::workspace::settings_ui::Preferences;

pub(in crate::workspace) struct ExportDraft {
    pub(super) format: ExportType,
    quality: Field,
    pub(super) dpi: Field,
    canvas: (u32, u32),
    pub(super) option_active: bool,
}

impl ExportDraft {
    fn new(preferences: &Preferences, canvas: (u32, u32)) -> Self {
        Self {
            format: preferences.export_type,
            quality: Field::new(preferences.jpeg_quality.to_string()),
            dpi: Field::new(preferences.pdf_export_dpi.to_string()),
            canvas,
            option_active: false,
        }
    }

    pub(super) fn prepare(&self) -> Result<ExportFormat> {
        Ok(match self.format {
            ExportType::Png => ExportFormat::Raster {
                format: FileFormat::Png,
                jpeg_quality: 0,
            },
            ExportType::WebP => ExportFormat::Raster {
                format: FileFormat::WebP,
                jpeg_quality: 0,
            },
            ExportType::Jpeg => {
                let quality = self.quality.text.parse::<u8>().unwrap_or(0);
                ensure!(
                    (1..=100).contains(&quality),
                    "Enter a quality from 1 to 100."
                );
                ExportFormat::Raster {
                    format: FileFormat::Jpeg,
                    jpeg_quality: quality,
                }
            }
            ExportType::Pdf => {
                let dpi = self.dpi.text.parse().unwrap_or(0);
                crate::pdf::export::page_size(self.canvas, dpi)?;
                ExportFormat::Pdf { dpi }
            }
        })
    }

    pub(super) fn handle_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Workspace>) {
        let key = &event.keystroke;
        if key.key == "tab" {
            self.option_active =
                !self.option_active && matches!(self.format, ExportType::Jpeg | ExportType::Pdf);
        } else if self.option_active {
            match self.format {
                ExportType::Jpeg => {
                    self.quality
                        .handle_key(event, FieldMode::Digits { max: 3 }, cx)
                }
                ExportType::Pdf => self.dpi.handle_key(event, FieldMode::Digits { max: 4 }, cx),
                _ => {}
            }
        } else if !key.modifiers.modified() && matches!(key.key.as_str(), "left" | "right") {
            let index = ExportType::ALL
                .iter()
                .position(|format| *format == self.format)
                .unwrap_or(0);
            self.format = ExportType::ALL[(index + if key.key == "right" { 1 } else { 3 }) % 4];
        }
    }
}

impl Workspace {
    pub(in crate::workspace) fn export_form(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dialog_open || self.form.is_some() || self.pdf_job.is_some() {
            return;
        }
        if self.commit_numeric_with_followup(
            crate::workspace::numeric::NumericFollowup::Export,
            window,
            cx,
        ) {
            return;
        }
        if self.finish_text(window, cx) {
            self.defer_after_text(Self::export_form);
            return;
        }
        if self.refuse_pending_file_edit(cx) {
            return;
        }
        let Some(tab) = self.active_tab().filter(|tab| !tab.busy()) else {
            return;
        };
        let draft = ExportDraft::new(&self.preferences, tab.document.content.size());
        self.show_form(FormKind::Export(draft), window, cx);
    }

    pub(super) fn export_controls(&self, draft: &ExportDraft, cx: &Context<Self>) -> Div {
        let formats = segmented(ExportType::ALL.into_iter().map(|format| {
            choice_button(format.label(), draft.format == format).on_click(cx.listener(
                move |this, _, window, cx| {
                    if let Some(form) = this.form.as_mut()
                        && let FormKind::Export(draft) = &mut form.kind
                    {
                        draft.format = format;
                        draft.option_active = false;
                        form.error = None;
                        form.pdf_input_drag = None;
                        form.focus.focus(window);
                        cx.notify();
                    }
                },
            ))
        }))
        .when(!draft.option_active, |row| row.border_color(rgb(ACCENT)));
        let options = match draft.format {
            ExportType::Png | ExportType::WebP => div()
                .text_sm().text_color(rgb(MUTED))
                .child("Lossless image with transparency. Visible layers and text are flattened."),
            ExportType::Jpeg => div().flex().flex_col().gap_1()
                .child("Quality (1–100)")
                .child(draft.quality.view(draft.option_active).id("export-quality")
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(form) = this.form.as_mut()
                            && let FormKind::Export(draft) = &mut form.kind
                        {
                            draft.option_active = true;
                            form.focus.focus(window);
                            cx.notify();
                        }
                    })))
                .child(div().text_xs().text_color(rgb(MUTED))
                    .child("Higher values keep more detail and make larger files. Transparent areas become white.")),
            ExportType::Pdf => {
                let size = crate::pdf::export::page_size(draft.canvas, draft.dpi.text.parse().unwrap_or(0));
                let valid = size.is_ok();
                let summary = match size {
                    Ok((width, height)) => format!(
                        "Page size: {:.1} × {:.1} mm ({:.2} × {:.2} in)",
                        width * 25.4 / 72., height * 25.4 / 72., width / 72., height / 72.,
                    ),
                    Err(error) => error.to_string(),
                };
                div().flex().flex_col().gap_3()
                    .child("Exports the current canvas as a single image-based PDF page.")
                    .child(div().child("Resolution (DPI)").child(
                        self.pdf_input_view(pdf_input::PdfInput::Dpi, &draft.dpi, draft.option_active, None, cx).mt_1()
                    ))
                    .child(div().text_sm().text_color(rgb(if valid { MUTED } else { 0xffb8ad })).child(summary))
                    .child(div().text_xs().text_color(rgb(MUTED)).child(format!(
                        "{} × {} pixels, lossless, white background. Resolution changes page size without resizing pixels. Text and visible layers are flattened.",
                        draft.canvas.0, draft.canvas.1,
                    )))
            }
        };
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(div().child("Format").child(formats.mt_1()))
            .child(options)
    }
}
