use super::{Workspace, settings_ui::Preferences};
use crate::commands::{ResetAllTools, ResetCurrentTool, TOOL_GROUPS, Tool};
use crate::settings::Patch;
use gpui::{Context, Div, PromptButton, PromptLevel, Window, prelude::*};

fn has_options(tool: Tool) -> bool {
    !matches!(tool, Tool::Pan | Tool::Zoom | Tool::Eyedropper)
}

impl Preferences {
    /// Only personal tool options belong here. This operation has no access to
    /// documents, color memory, gradient libraries, or active editing sessions.
    pub(super) fn reset_tool(&mut self, tool: Tool) -> Vec<&'static str> {
        let defaults = Self::default();
        let mut paths = Vec::new();
        macro_rules! reset {
            ($field:ident) => {{
                self.$field = defaults.$field;
                paths.push(stringify!($field));
            }};
        }
        match tool {
            Tool::Move => {
                reset!(snap_grid);
                reset!(snap_guides);
                reset!(grid_spacing);
            }
            Tool::Select | Tool::OvalSelect => {
                reset!(selection_ratio);
                reset!(selection_ratio_value);
                reset!(selection_antialias);
                reset!(oval_circle);
                if tool == Tool::Select {
                    reset!(selection_radius);
                }
            }
            Tool::FreeSelect => reset!(free_antialias),
            Tool::MagicWand => {
                reset!(wand_match);
                reset!(wand_sample_all);
                reset!(wand_antialias);
            }
            Tool::Crop => {
                reset!(crop_ratio);
                reset!(crop_ratio_value);
            }
            Tool::ScaleLayer => {
                reset!(scale_lock);
                reset!(scale_anchor);
                reset!(warp_resample);
            }
            Tool::Rotate | Tool::Flip | Tool::Perspective => {
                reset!(warp_resample)
            }
            Tool::Pencil
            | Tool::Brush
            | Tool::Eraser
            | Tool::Blur
            | Tool::Sharpen
            | Tool::CloneStamp => {
                paths.extend(self.brushes.reset_tool(tool));
                if tool == Tool::CloneStamp {
                    reset!(clone_aligned);
                    reset!(clone_sample_all);
                    reset!(clone_show_source);
                }
            }
            Tool::BucketFill => {
                reset!(fill_mode);
                reset!(fill_match);
                reset!(sample_all);
            }
            Tool::Gradient => {
                reset!(gradient_shape);
                reset!(gradient_blend);
                reset!(gradient_custom);
                reset!(gradient_use_custom);
                reset!(gradient_reverse);
                reset!(gradient_selected_stop);
                reset!(gradient_selected_preset);
            }
            Tool::Text => reset!(text_defaults),
            Tool::Path => {
                reset!(path_handle_mode);
                reset!(path_stroke_style);
                paths.extend(self.brushes.reset_tool(tool));
            }
            Tool::Pan | Tool::Zoom | Tool::Eyedropper => {}
        }
        if matches!(
            tool,
            Tool::Select | Tool::OvalSelect | Tool::FreeSelect | Tool::MagicWand
        ) {
            reset!(selection_feather_enabled);
            reset!(selection_feather_radius);
        }
        paths
    }

    pub(super) fn reset_all_tools(&mut self) -> Vec<&'static str> {
        let mut paths = Vec::new();
        for (_, tools) in TOOL_GROUPS {
            for &tool in *tools {
                paths.extend(self.reset_tool(tool));
            }
        }
        paths
    }
}

impl Workspace {
    pub(super) fn tool_reset_blocker(&self) -> Option<&'static str> {
        if self.settings_store.is_loading() {
            Some("Wait for saved settings to finish loading.")
        } else if self.form.is_some()
            || self.dialog_open
            || self.pdf_job.is_some()
            || self.creating
            || self.tabs.iter().any(|tab| tab.busy())
            || self.gradient_store.saving
        {
            Some("Finish the current dialog or background operation before resetting tools.")
        } else if self.numeric_edit.is_some()
            || self.numeric_followup.is_some()
            || self.text_draft.is_some()
            || self.toolbox_drag.is_some()
            || self.pan_drag.is_some()
            || self.pending_nudge.is_some()
            || self.pending_edit_name().is_some()
        {
            Some("Finish or cancel the current edit or preview before resetting tools.")
        } else {
            None
        }
    }

    pub(super) fn current_tool_reset_blocker(&self) -> Option<&'static str> {
        self.tool_reset_blocker().or_else(|| {
            (!has_options(self.preferences.tool)).then_some("This tool has no settings to reset.")
        })
    }

    pub(super) fn bind_tool_reset_actions(&self, root: Div, cx: &Context<Self>) -> Div {
        root.when(self.current_tool_reset_blocker().is_none(), |root| {
            root.on_action(cx.listener(|this, _: &ResetCurrentTool, window, cx| {
                this.reset_tool_settings(false, window, cx);
            }))
        })
        .when(self.tool_reset_blocker().is_none(), |root| {
            root.on_action(cx.listener(|this, _: &ResetAllTools, window, cx| {
                this.confirm_reset_all_tools(window, cx);
            }))
        })
    }

    fn confirm_reset_all_tools(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.tool_reset_blocker().is_some() {
            return;
        }
        self.dialog_open = true;
        let prompt = window.prompt(
            PromptLevel::Warning,
            "Reset all tool settings to their defaults?",
            Some("Resets every tool, including shared options. Keeps artwork, colors, saved gradients, workspace layout, import/export preferences, and filter settings."),
            &[PromptButton::cancel("Cancel"), PromptButton::new("Reset All Tools")],
            cx,
        );
        cx.spawn_in(window, async move |workspace, cx| {
            let answer = prompt.await;
            let _ = workspace.update_in(cx, |this, window, cx| {
                this.dialog_open = false;
                if answer == Ok(1) {
                    this.reset_tool_settings(true, window, cx);
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    fn reset_tool_settings(&mut self, all: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.tool_reset_blocker().is_some() || (!all && !has_options(self.preferences.tool)) {
            return;
        }
        let mut candidate = self.preferences.clone();
        let paths = if all {
            candidate.reset_all_tools()
        } else {
            candidate.reset_tool(self.preferences.tool)
        };
        let mut reset = Patch::default();
        if let Err(error) = reset.replace(&candidate, &paths) {
            self.errors
                .push(format!("Could not reset tool settings: {error:#}"));
            return;
        }
        // Preserve other changes before the explicit reset replaces its entire
        // scope, including queued leaves and locally unchanged default values.
        self.sync_settings(window, cx);
        self.queue_settings_patch(&reset);
        // Use Clone's existing source/alignment invalidation rules. Raster data
        // and the chosen source remain intact when its policy is unchanged.
        self.set_clone_sampling(candidate.clone_sample_all, cx);
        self.set_clone_aligned(candidate.clone_aligned, cx);
        self.preferences = candidate;
        if all || self.preferences.tool == Tool::Gradient {
            self.gradient_revision = self.gradient_revision.wrapping_add(1);
        }
        self.flush_settings(window, cx);
        self.notice(
            if all {
                "All tool settings reset to defaults."
            } else {
                "Tool defaults reset. Existing selections and layers are unchanged."
            },
            cx,
        );
        cx.notify();
    }
}
