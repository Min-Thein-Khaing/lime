use gpui::{App, Div, Stateful, div, prelude::*, px, rgb};

use super::UpdateState;
use crate::theme::{ACCENT, HOVER, SELECTED};

pub(crate) fn badge(cx: &App) -> Option<Stateful<Div>> {
    let update = cx.try_global::<UpdateState>()?.available.as_ref()?;
    Some(
        div()
            .id("update-available")
            .tab_index(0)
            .flex_shrink_0()
            .ml_4()
            .px_2()
            .py(px(2.))
            .rounded_md()
            .bg(rgb(SELECTED))
            .text_color(rgb(ACCENT))
            .text_size(px(11.))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(HOVER)))
            .child(format!("↑ Lime {} available", update.version))
            .on_click(|_, _, cx| show(cx))
            .on_key_down(|event, _, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    show(cx);
                    cx.stop_propagation();
                }
            }),
    )
}

pub(crate) fn show(cx: &mut App) {
    cx.defer(|cx| {
        if let Ok(handle) = crate::ensure_workspace(cx) {
            let _ = handle.update(cx, |workspace, window, cx| {
                workspace.show_updates(window, cx);
            });
        }
    });
}
