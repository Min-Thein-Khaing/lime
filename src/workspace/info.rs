//! About and update status share the editor's modal host, never another OS window.
use gpui::{
    Context, Div, FocusHandle, Keystroke, MouseButton, Window, div, prelude::*, px, rgb, rgba,
};

use super::Workspace;
use super::dialogs::FormKind;
use crate::theme::{ACCENT, BORDER, CHROME, HOVER, MUTED, PANEL, SELECTED, TEXT};
use crate::updates::{self, CheckStatus, UpdateState};

const SOURCE: &str = env!("CARGO_PKG_REPOSITORY");
const ISSUES: &str = "https://github.com/eimg/lime/issues";

#[derive(Clone, Copy)]
pub(super) enum InfoKind {
    About,
    Updates,
}

pub(super) struct InfoDialog {
    kind: InfoKind,
    pub(super) prior_focus: Option<FocusHandle>,
    buttons: [FocusHandle; 4],
}

#[derive(Clone)]
enum InfoAction {
    Close,
    Link(String),
    Check,
}

// Stable slots keep Close focused when a completed check inserts new links.
fn button_slot(label: &str, action: &InfoAction) -> usize {
    match action {
        InfoAction::Close => 0,
        InfoAction::Check => 3,
        InfoAction::Link(_) if matches!(label, "Source code ↗" | "View changelog ↗") => 1,
        InfoAction::Link(_) => 2,
    }
}

struct Content {
    title: String,
    detail: String,
    notes: Vec<String>,
    actions: Vec<(&'static str, InfoAction)>,
}

fn content(kind: InfoKind, state: &UpdateState) -> Content {
    if matches!(kind, InfoKind::About) {
        return Content {
            title: "About Lime".into(),
            detail: format!(
                "Lime — Lite Image Editor\nVersion {}",
                env!("CARGO_PKG_VERSION")
            ),
            notes: vec![format!(
                "{} · {}",
                match std::env::consts::OS {
                    "macos" => "macOS",
                    "windows" => "Windows",
                    "linux" => "Linux",
                    other => other,
                },
                match std::env::consts::ARCH {
                    "aarch64" => "ARM64",
                    "x86_64" => "x64",
                    other => other,
                }
            )],
            actions: vec![
                ("Source code ↗", InfoAction::Link(SOURCE.into())),
                ("Report an issue ↗", InfoAction::Link(ISSUES.into())),
                ("Close", InfoAction::Close),
            ],
        };
    }
    let mut notes = Vec::new();
    let mut actions = Vec::new();
    let (title, detail) = if let Some(update) = &state.available {
        actions.push((
            "View changelog ↗",
            InfoAction::Link(update.changelog.clone()),
        ));
        if let Some(url) = &update.download {
            // The same wording covers every supported architecture and platform.
            actions.push(("Download ↗", InfoAction::Link(url.clone())));
        } else {
            notes.push("A download for your platform hasn't been published yet.".into());
        }
        if state.checking {
            notes.push("Checking for newer releases…".into());
        } else if matches!(state.status, CheckStatus::Failed) {
            notes.push("The latest check failed. Showing the last available release.".into());
        }
        (
            format!("Lime {} is available", update.version),
            format!(
                "You're using Lime {}. Download the new version and install it when you're ready.",
                env!("CARGO_PKG_VERSION")
            ),
        )
    } else if state.checking {
        (
            "Checking for updates…".into(),
            "Looking for the latest stable release of Lime.".into(),
        )
    } else if matches!(state.status, CheckStatus::Current) {
        (
            "You're up to date".into(),
            format!(
                "Lime {} is the latest stable version.",
                env!("CARGO_PKG_VERSION")
            ),
        )
    } else if matches!(state.status, CheckStatus::Failed) {
        actions.push(("Releases ↗", InfoAction::Link(updates::RELEASES.into())));
        (
            "Couldn't check for updates".into(),
            "Check your internet connection and try again, or browse the releases on GitHub."
                .into(),
        )
    } else {
        (
            "Lime Updates".into(),
            format!("You're using Lime {}.", env!("CARGO_PKG_VERSION")),
        )
    };
    if !state.checking {
        actions.push(("Check again", InfoAction::Check));
    }
    actions.push(("Close", InfoAction::Close));
    Content {
        title,
        detail,
        notes,
        actions,
    }
}

impl Workspace {
    pub(crate) fn show_about(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_information(InfoKind::About, window, cx);
    }

    pub(crate) fn show_updates(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_information(InfoKind::Updates, window, cx);
    }

    fn show_information(&mut self, kind: InfoKind, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(form) = self.form.as_mut()
            && let FormKind::Info(info) = &mut form.kind
        {
            info.kind = kind;
            form.focus.focus(window);
            cx.notify();
            return;
        }
        // A committed inline field disappears from the focus tree. Return to
        // the editor rather than retaining that now-unrendered focus handle.
        if self.numeric_focus.is_focused(window) {
            self.commit_numeric_edit(window, cx);
            self.focus.focus(window);
        }
        if self.form.is_some()
            || self.dialog_open
            || self.creating
            || self.pdf_job.is_some()
            || self.pending_canvas_edit_name() == Some("gesture")
        {
            return;
        }
        self.close_font_picker(window, cx);
        self.layer_menu = None;
        #[cfg(any(target_os = "linux", test))]
        self.linux_menu.close();
        let info = InfoDialog {
            kind,
            prior_focus: window.focused(cx),
            buttons: std::array::from_fn(|_| cx.focus_handle()),
        };
        self.show_form(FormKind::Info(info), window, cx);
    }

    fn information_action(
        &mut self,
        action: InfoAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match action {
            InfoAction::Close => self.dismiss_form(window, cx),
            InfoAction::Link(url) => cx.open_url(&url),
            InfoAction::Check => updates::check(true, cx),
        }
    }

    // Intercept before inline editors and global shortcuts. Explicitly cycle only
    // the dialog's live buttons, so Tab cannot escape into the toolbox/canvas.
    pub(super) fn info_key(
        &mut self,
        key: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(form) = &self.form else { return false };
        let FormKind::Info(info) = &form.kind else {
            return false;
        };
        let actions = content(info.kind, cx.global::<UpdateState>()).actions;
        let selected = actions.iter().position(|(label, action)| {
            info.buttons[button_slot(label, action)].is_focused(window)
        });
        match key.key.as_str() {
            "escape" => self.dismiss_form(window, cx),
            "tab" => {
                let index = next_button(selected, actions.len(), key.modifiers.shift);
                let (label, action) = &actions[index];
                info.buttons[button_slot(label, action)].focus(window);
            }
            "enter" | "space" if !key.modifiers.modified() => {
                let index = selected.unwrap_or(actions.len() - 1);
                self.information_action(actions[index].1.clone(), window, cx);
            }
            _ => {}
        }
        true
    }

    pub(super) fn info_overlay(
        &self,
        info: &InfoDialog,
        focus: &FocusHandle,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let content = content(info.kind, cx.global::<UpdateState>());
        div()
            .absolute()
            .size_full()
            .occlude()
            .bg(rgba(0x0b0d12bb))
            .flex()
            .items_center()
            .justify_center()
            .p_4()
            .track_focus(focus)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
            .on_scroll_wheel(|_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .id("information-dialog")
                    .w(px(480.))
                    .max_w_full()
                    .max_h((window.viewport_size().height - px(32.)).max(px(0.)))
                    .overflow_y_scroll()
                    .p_5()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(PANEL))
                    .text_color(rgb(TEXT))
                    .shadow_lg()
                    .child(div().text_size(px(23.)).child(content.title))
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(MUTED))
                            .child(content.detail),
                    )
                    .children(
                        content.notes.into_iter().map(|note| {
                            div().text_size(px(12.)).text_color(rgb(MUTED)).child(note)
                        }),
                    )
                    .child(div().flex().flex_wrap().gap_2().mt_2().children(
                        content.actions.into_iter().map(|(label, action)| {
                            let close = matches!(action, InfoAction::Close);
                            let slot = button_slot(label, &action);
                            div()
                                .id(("information-button", slot))
                                .track_focus(&info.buttons[slot])
                                .tab_index(0)
                                .px_3()
                                .py_2()
                                .rounded_md()
                                .border_1()
                                .border_color(rgb(if close { ACCENT } else { BORDER }))
                                .bg(rgb(if close { SELECTED } else { CHROME }))
                                .text_size(px(13.))
                                .cursor_pointer()
                                .hover(|button| button.bg(rgb(HOVER)))
                                .focus(|button| button.border_color(rgb(ACCENT)))
                                .child(label)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.information_action(action.clone(), window, cx);
                                }))
                        }),
                    )),
            )
    }
}

fn next_button(selected: Option<usize>, count: usize, backwards: bool) -> usize {
    match (selected, backwards) {
        (Some(index), true) => (index + count - 1) % count,
        (Some(index), false) => (index + 1) % count,
        (None, true) => count - 1,
        (None, false) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_focus_wraps_within_its_controls() {
        assert_eq!(next_button(None, 3, false), 0);
        assert_eq!(next_button(None, 3, true), 2);
        assert_eq!(next_button(Some(0), 3, true), 2);
        assert_eq!(next_button(Some(2), 3, false), 0);
    }

    #[test]
    fn update_controls_cover_missing_downloads_stale_results_and_background_checks() {
        let mut state = UpdateState {
            available: Some(updates::Available {
                version: "99.0.0".into(),
                changelog: SOURCE.into(),
                download: None,
            }),
            status: CheckStatus::Failed,
            ..Default::default()
        };
        let failed = content(InfoKind::Updates, &state);
        assert!(
            failed
                .notes
                .iter()
                .any(|note| note.contains("hasn't been published"))
        );
        assert!(
            failed
                .notes
                .iter()
                .any(|note| note.contains("last available release"))
        );
        assert!(
            !failed
                .actions
                .iter()
                .any(|(label, _)| *label == "Download ↗")
        );
        state.available.as_mut().unwrap().download = Some(SOURCE.into());
        state.checking = true;
        let checking = content(InfoKind::Updates, &state);
        assert!(checking.actions.len() <= 4);
        for panel in [&failed, &checking] {
            let slots: Vec<_> = panel
                .actions
                .iter()
                .map(|(label, action)| button_slot(label, action))
                .collect();
            assert_eq!(
                slots.len(),
                slots.iter().collect::<std::collections::HashSet<_>>().len()
            );
            assert_eq!(*slots.last().unwrap(), 0); // Close never moves when state changes.
        }
        assert!(
            checking
                .actions
                .iter()
                .any(|(label, _)| *label == "Download ↗")
        );
        assert!(
            !checking
                .actions
                .iter()
                .any(|(_, action)| matches!(action, InfoAction::Check))
        );
    }
}
