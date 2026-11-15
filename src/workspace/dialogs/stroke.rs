//! Menu strokes share one set of options and the panel's existing stroke engine.
use anyhow::{Result, ensure};
use gpui::{Context, Div, KeyDownEvent, Window, div, prelude::*, rgb};

use super::{
    ACCENT, Field, FieldMode, FormKind, MUTED, Workspace, choice_button, form_button, segmented,
};
use crate::document::LayerId;
use crate::paint::Brush;
use crate::paths::PathId;
use crate::workspace::stroke_ui::{PathStrokeStyle, StrokeSource};
use crate::workspace::{Tab, numeric::NumericFollowup};

const LABELS: [&str; 5] = [
    "Size (px)",
    "Spacing (%)",
    "Hardness (%)",
    "Opacity (%)",
    "Flow (%)",
];

#[derive(Clone, Copy)]
struct StrokeTarget {
    tab: usize,
    layer: LayerId,
    generation: u64,
    path: Option<PathId>,
}

impl StrokeTarget {
    fn capture(tab: &Tab, source: StrokeSource) -> Self {
        Self {
            tab: tab.id,
            layer: tab.active_layer,
            generation: tab.history.generation(),
            path: if source == StrokeSource::Path {
                tab.document.content.active_path().map(|path| path.id)
            } else {
                None
            },
        }
    }

    fn matches(self, tab: &Tab) -> bool {
        self.tab == tab.id
            && self.layer == tab.active_layer
            && self.generation == tab.history.generation()
            && !tab.busy()
            && self.path.is_none_or(|id| {
                tab.document
                    .content
                    .active_path()
                    .is_some_and(|path| path.id == id)
            })
    }
}

pub(in crate::workspace) struct StrokeDraft {
    source: StrokeSource,
    target: StrokeTarget,
    style: PathStrokeStyle,
    fields: [Field; 5],
    buildup: bool,
    active: usize, // style, five numeric fields, buildup
}

impl StrokeDraft {
    fn new(
        source: StrokeSource,
        target: StrokeTarget,
        style: PathStrokeStyle,
        brush: Brush,
    ) -> Self {
        Self {
            source,
            target,
            style,
            fields: [
                brush.diameter,
                u32::from(brush.spacing),
                u32::from(brush.hardness),
                u32::from(brush.opacity),
                u32::from(brush.flow),
            ]
            .map(|value| Field::new(value.to_string())),
            buildup: brush.buildup,
            active: 1,
        }
    }

    pub(super) fn title(&self) -> &'static str {
        match self.source {
            StrokeSource::Path => "Stroke Path",
            StrokeSource::Selection => "Stroke Selection",
        }
    }

    pub(super) fn prepare(&self) -> Result<Brush> {
        let mut values = [0; 5];
        for (index, field) in self.fields.iter().enumerate() {
            values[index] = field.text.parse::<u32>().unwrap_or(u32::MAX);
            let (min, max) = match index {
                0 => (1, 500),
                1 => (1, 100),
                _ => (0, 100),
            };
            ensure!(
                (min..=max).contains(&values[index]),
                "{} must be from {min} to {max}.",
                LABELS[index]
            );
        }
        Ok(Brush {
            diameter: values[0],
            spacing: values[1] as u8,
            hardness: values[2] as u8,
            opacity: values[3] as u8,
            flow: values[4] as u8,
            buildup: self.buildup,
        })
    }

    pub(super) fn handle_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Workspace>) {
        let key = &event.keystroke;
        if key.key == "tab" {
            self.active = (self.active + if key.modifiers.shift { 6 } else { 1 }) % 7;
        } else if self.active == 0
            && !key.modifiers.modified()
            && matches!(key.key.as_str(), "left" | "right")
        {
            self.style = if self.style == PathStrokeStyle::Pencil {
                PathStrokeStyle::Brush
            } else {
                PathStrokeStyle::Pencil
            };
        } else if self.active == 6 && key.key == "space" && !key.modifiers.modified() {
            self.buildup = !self.buildup;
        } else if (1..=5).contains(&self.active) {
            self.fields[self.active - 1].handle_key(event, FieldMode::Digits { max: 3 }, cx);
        }
    }
}

impl Workspace {
    pub(in crate::workspace) fn stroke_form(
        &mut self,
        source: StrokeSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.form.is_some() || self.dialog_open || self.pdf_job.is_some() {
            return;
        }
        let Some(tab) = self.active_tab() else { return };
        let layer = tab.active_layer;
        let path = if source == StrokeSource::Path {
            tab.document.content.active_path().map(|path| path.id)
        } else {
            None
        };
        if self.commit_numeric_with_followup(
            NumericFollowup::Stroke {
                source,
                dialog: true,
                layer,
                path,
            },
            window,
            cx,
        ) {
            return;
        }
        if self.finish_text(window, cx) {
            self.defer_after_text(move |this, window, cx| this.stroke_form(source, window, cx));
            return;
        }
        if self.refuse_pending_file_edit(cx) || !self.can_stroke_outline(source) {
            return;
        }
        let Some(tab) = self.active_tab() else { return };
        let draft = StrokeDraft::new(
            source,
            StrokeTarget::capture(tab, source),
            self.preferences.path_stroke_style,
            self.preferences.brushes.outline,
        );
        self.show_form(FormKind::Stroke(draft), window, cx);
    }

    pub(super) fn submit_stroke_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(form) = &self.form else { return };
        let FormKind::Stroke(draft) = &form.kind else {
            return;
        };
        let result = draft.prepare().and_then(|brush| {
            ensure!(
                self.active_tab()
                    .is_some_and(|tab| draft.target.matches(tab))
                    && self.can_stroke_outline(draft.source),
                "The stroke target changed. Close this dialog and try again."
            );
            Ok((draft.source, draft.style, brush))
        });
        match result {
            Ok((source, style, brush)) => {
                self.dismiss_form(window, cx);
                self.preferences.path_stroke_style = style;
                self.preferences.brushes.outline = brush;
                self.stroke_outline(source, window, cx);
            }
            Err(error) => {
                if let Some(form) = &mut self.form {
                    form.error = Some(error.to_string());
                }
                cx.notify();
            }
        }
    }

    pub(super) fn stroke_controls(&self, draft: &StrokeDraft, cx: &Context<Self>) -> Div {
        let style = segmented(
            [
                (PathStrokeStyle::Pencil, "Pencil"),
                (PathStrokeStyle::Brush, "Brush"),
            ]
            .map(|(style, label)| {
                choice_button(label, draft.style == style).on_click(cx.listener(
                    move |this, _, _, cx| {
                        if let Some(form) = &mut this.form
                            && let FormKind::Stroke(draft) = &mut form.kind
                        {
                            draft.style = style;
                            draft.active = 0;
                            cx.notify();
                        }
                    },
                ))
            }),
        )
        .when(draft.active == 0, |row| row.border_color(rgb(ACCENT)));
        div().flex().flex_col().gap_3()
            .child(div().text_sm().text_color(rgb(MUTED)).child("Paints onto the active layer with the foreground color. Path and selection strokes share these settings."))
            .child(style)
            .children((0..3).map(|row| div().flex().gap_3().children((row * 2..(row * 2 + 2).min(5)).map(|index| {
                div().flex_1().min_w_0().child(LABELS[index]).child(draft.fields[index].view(draft.active == index + 1).id(("stroke-field", index)).mt_1().on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(form) = &mut this.form && let FormKind::Stroke(draft) = &mut form.kind { draft.active = index + 1; cx.notify(); }
                })))
            }))))
            .child(form_button("Buildup", draft.buildup).when(draft.active == 6, |button| button.border_color(rgb(ACCENT))).on_click(cx.listener(|this, _, _, cx| {
                if let Some(form) = &mut this.form && let FormKind::Stroke(draft) = &mut form.kind { draft.buildup = !draft.buildup; draft.active = 6; cx.notify(); }
            })))
            .child(div().text_xs().text_color(rgb(MUTED)).child("Flow applies when Buildup is enabled. Pencil strokes use hard edges."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stroke_target_cannot_follow_a_changed_path_layer_or_document_revision() {
        use crate::paths::{Anchor, AnchorId, VectorPath};
        let mut tab = crate::workspace::tests::opacity_tab();
        let path = VectorPath {
            id: PathId(1),
            visible: true,
            closed: false,
            anchors: vec![
                Anchor::corner(AnchorId(1), (0., 0.)),
                Anchor::corner(AnchorId(2), (1., 1.)),
            ],
        };
        assert!(tab.document.content.upsert_path(path.clone()));
        let target = StrokeTarget::capture(&tab, StrokeSource::Path);
        assert!(target.matches(&tab));
        let mut other = path;
        other.id = PathId(2);
        for anchor in &mut other.anchors {
            anchor.id = AnchorId(anchor.id.0 + 2);
        }
        assert!(tab.document.content.upsert_path(other));
        assert!(!target.matches(&tab));
        assert!(tab.document.content.select_path(PathId(1)));
        assert!(target.matches(&tab));
        let layer = tab.active_layer;
        tab.active_layer = LayerId(layer.0 + 1);
        assert!(!target.matches(&tab));
        tab.active_layer = layer;
        let state = tab.state();
        tab.history.commit(state, 1, false);
        assert!(!target.matches(&tab));
    }

    #[test]
    fn stroke_dialog_rejects_stale_targets_and_preserves_the_shared_brush_until_acceptance() {
        let mut tab = crate::workspace::tests::opacity_tab();
        let brush = Brush::default();
        let target = StrokeTarget::capture(&tab, StrokeSource::Selection);
        let mut draft = StrokeDraft::new(
            StrokeSource::Selection,
            target,
            PathStrokeStyle::Brush,
            brush,
        );
        draft.fields[0] = Field::new("501".into());
        assert!(draft.prepare().is_err());
        draft.fields[0] = Field::new("24".into());
        assert_eq!(draft.prepare().unwrap().diameter, 24);
        assert_eq!(brush.diameter, 16);
        assert!(target.matches(&tab));
        tab.editing = true;
        assert!(!target.matches(&tab));
        tab.editing = false;
        tab.id += 1;
        assert!(!target.matches(&tab));
    }
}
