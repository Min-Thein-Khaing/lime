use super::{InspectorTab, Workspace, effects, paint_ui, pdf_ui, stroke_ui, text_ui, view};
use crate::color_memory::Changes;
use crate::settings::{self, Patch};
use crate::{
    commands::Tool,
    document::{DEFAULT_JPEG_QUALITY, FillMode},
};
use anyhow::{Result, ensure};
use gpui::{Context, Window};
use std::{path::PathBuf, time::Duration};

/// Disclosure state is workspace layout, independent of tool settings/reset.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct ToolSections {
    edges: bool,
    stroke: bool,
    clone_source: bool,
    text_spacing: bool,
    text_layout: bool,
    rotation_pivot: bool,
    perspective_corner: bool,
    gradient_stops: bool,
}

impl Default for ToolSections {
    fn default() -> Self {
        Self {
            edges: false,
            stroke: false,
            clone_source: false,
            text_spacing: false,
            text_layout: false,
            rotation_pivot: false,
            perspective_corner: false,
            // Custom mode should expose its editor on first use.
            gradient_stops: true,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum ToolSection {
    Edges,
    Stroke,
    CloneSource,
    TextSpacing,
    TextLayout,
    RotationPivot,
    PerspectiveCorner,
    GradientStops,
}

impl ToolSection {
    fn value(self, sections: &mut ToolSections) -> &mut bool {
        match self {
            Self::Edges => &mut sections.edges,
            Self::Stroke => &mut sections.stroke,
            Self::CloneSource => &mut sections.clone_source,
            Self::TextSpacing => &mut sections.text_spacing,
            Self::TextLayout => &mut sections.text_layout,
            Self::RotationPivot => &mut sections.rotation_pivot,
            Self::PerspectiveCorner => &mut sections.perspective_corner,
            Self::GradientStops => &mut sections.gradient_stops,
        }
    }

    pub(super) fn expanded(self, sections: &ToolSections) -> bool {
        match self {
            Self::Edges => sections.edges,
            Self::Stroke => sections.stroke,
            Self::CloneSource => sections.clone_source,
            Self::TextSpacing => sections.text_spacing,
            Self::TextLayout => sections.text_layout,
            Self::RotationPivot => sections.rotation_pivot,
            Self::PerspectiveCorner => sections.perspective_corner,
            Self::GradientStops => sections.gradient_stops,
        }
    }

    pub(super) fn toggle(self, sections: &mut ToolSections) {
        let value = self.value(sections);
        *value = !*value;
    }
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct Preferences {
    pub(super) tool: Tool,
    pub(super) show_grid: bool,
    pub(super) show_guides: bool,
    pub(super) snap_grid: bool,
    pub(super) snap_guides: bool,
    pub(super) grid_spacing: u32,
    pub(super) show_layer_bounds: bool,
    // Accepted from older settings and normalized to the shared 1:1 ratio.
    pub(super) oval_circle: bool,
    pub(super) selection_radius: u32,
    pub(super) selection_antialias: bool,
    pub(super) free_antialias: bool,
    pub(super) wand_antialias: bool,
    pub(super) selection_feather_enabled: bool,
    pub(super) selection_feather_radius: u32,
    pub(super) selection_ratio: Option<crate::constraints::AspectRatio>,
    pub(super) selection_ratio_value: crate::constraints::AspectRatio,
    pub(super) crop_ratio: Option<crate::constraints::AspectRatio>,
    pub(super) crop_ratio_value: crate::constraints::AspectRatio,
    pub(super) wand_match: crate::selection::RegionMatch,
    pub(super) fill_match: crate::selection::RegionMatch,
    pub(super) scale_lock: bool,
    pub(super) scale_anchor: crate::transform::Anchor,
    pub(super) warp_resample: crate::transform::Resample,
    pub(super) brushes: paint_ui::BrushSettings,
    pub(super) fill_mode: FillMode,
    pub(super) sample_all: bool,
    pub(super) wand_sample_all: bool,
    pub(super) gradient_shape: crate::paint::GradientShape,
    pub(super) gradient_blend: crate::paint::GradientBlend,
    pub(super) gradient_custom: Option<crate::gradient::ColorRamp>,
    pub(super) gradient_use_custom: bool,
    pub(super) gradient_reverse: bool,
    pub(super) gradient_selected_stop: usize,
    pub(super) gradient_presets_expanded: bool,
    pub(super) gradient_selected_preset: Option<u64>,
    pub(super) clone_aligned: bool,
    pub(super) clone_sample_all: bool,
    pub(super) clone_show_source: bool,
    pub(super) inspector_tab: InspectorTab,
    pub(super) inspector_folded: bool,
    pub(super) path_handle_mode: crate::paths::HandleMode,
    pub(super) path_stroke_style: stroke_ui::PathStrokeStyle,
    pub(super) text_defaults: text_ui::TextDefaults,
    pub(super) pdf_preferences: pdf_ui::PdfPreferences,
    pub(super) export_type: super::files::ExportType,
    pub(super) jpeg_quality: u8,
    pub(super) pdf_export_dpi: u32,
    pub(super) toolbox_width: f32,
    pub(super) toolbox_folded: bool,
    pub(super) move_snapping_expanded: bool,
    pub(super) tool_sections: ToolSections,
    pub(super) effects: effects::Preferences,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            tool: Tool::Move,
            show_grid: false,
            show_guides: false,
            snap_grid: false,
            snap_guides: false,
            grid_spacing: 16,
            show_layer_bounds: false,
            oval_circle: false,
            selection_radius: 0,
            selection_antialias: false,
            free_antialias: false,
            wand_antialias: false,
            selection_feather_enabled: false,
            selection_feather_radius: 8,
            selection_ratio: None,
            selection_ratio_value: crate::constraints::AspectRatio::SQUARE,
            crop_ratio: None,
            crop_ratio_value: crate::constraints::AspectRatio::SQUARE,
            wand_match: Default::default(),
            fill_match: Default::default(),
            scale_lock: true,
            scale_anchor: crate::transform::Anchor::Center,
            warp_resample: crate::transform::Resample::Bilinear,
            brushes: paint_ui::BrushSettings::default(),
            fill_mode: FillMode::Connected,
            sample_all: false,
            wand_sample_all: false,
            gradient_shape: crate::paint::GradientShape::Linear,
            gradient_blend: crate::paint::GradientBlend::ForegroundToBackground,
            gradient_custom: None,
            gradient_use_custom: false,
            gradient_reverse: false,
            gradient_selected_stop: 0,
            gradient_presets_expanded: false,
            gradient_selected_preset: None,
            clone_aligned: true,
            clone_sample_all: false,
            clone_show_source: true,
            inspector_tab: InspectorTab::Layers,
            inspector_folded: false,
            path_handle_mode: crate::paths::HandleMode::Independent,
            path_stroke_style: stroke_ui::PathStrokeStyle::Brush,
            text_defaults: text_ui::TextDefaults::default(),
            pdf_preferences: Default::default(),
            export_type: Default::default(),
            jpeg_quality: DEFAULT_JPEG_QUALITY,
            pdf_export_dpi: crate::pdf::export::DEFAULT_DPI,
            toolbox_width: view::TOOLBOX_DEFAULT_WIDTH,
            toolbox_folded: false,
            move_snapping_expanded: false,
            tool_sections: ToolSections::default(),
            effects: Default::default(),
        }
    }
}

impl Preferences {
    /// Called only after the export writer succeeds, never for dialog drafts.
    pub(super) fn remember_export(&mut self, format: super::files::ExportFormat) {
        use super::files::{ExportFormat, ExportType};
        match format {
            ExportFormat::Raster {
                format,
                jpeg_quality,
            } => {
                self.export_type = match format {
                    crate::document::FileFormat::Png => ExportType::Png,
                    crate::document::FileFormat::Jpeg => {
                        self.jpeg_quality = jpeg_quality;
                        ExportType::Jpeg
                    }
                    crate::document::FileFormat::WebP => ExportType::WebP,
                    crate::document::FileFormat::Project => {
                        unreachable!("projects are never exported")
                    }
                };
            }
            ExportFormat::Pdf { dpi } => {
                self.export_type = ExportType::Pdf;
                self.pdf_export_dpi = dpi;
            }
        }
    }

    /// Inline numeric previews and a toolbox resize become preferences only
    /// after their normal commit. Other settings can still save meanwhile.
    fn committed(
        &self,
        observed: &Self,
        field: Option<super::numeric::NumericField>,
        resizing_toolbox: bool,
    ) -> Self {
        use super::numeric::NumericField::*;
        let mut value = self.clone();
        if resizing_toolbox {
            value.toolbox_width = observed.toolbox_width;
        }
        match field {
            Some(
                BrushSize | BrushSpacing | BrushHardness | BrushOpacity | BrushFlow
                | PathStrokeSize,
            ) => value.brushes = observed.brushes.clone(),
            Some(TextSize | TextBoxWidth | TextBoxHeight | TextLetterSpacing | TextLineSpacing) => {
                value.text_defaults = observed.text_defaults.clone()
            }
            Some(SelectionRadius) => value.selection_radius = observed.selection_radius,
            Some(SelectionFeatherRadius) => {
                value.selection_feather_radius = observed.selection_feather_radius
            }
            Some(SelectionRatioWidth | SelectionRatioHeight) => {
                value.selection_ratio = observed.selection_ratio;
                value.selection_ratio_value = observed.selection_ratio_value;
            }
            Some(CropRatioWidth | CropRatioHeight) => {
                value.crop_ratio = observed.crop_ratio;
                value.crop_ratio_value = observed.crop_ratio_value;
            }
            Some(WandTolerance | WandGapRadius) => value.wand_match = observed.wand_match,
            Some(FillTolerance | FillGapRadius) => value.fill_match = observed.fill_match,
            Some(GridSpacing) => value.grid_spacing = observed.grid_spacing,
            Some(GradientStopPosition | GradientStopOpacity) => {
                value.gradient_custom = observed.gradient_custom
            }
            _ => {}
        }
        value
    }
}

impl settings::Preferences for Preferences {
    fn merge_groups() -> &'static [&'static [&'static str]] {
        &[
            &["selection_ratio", "oval_circle"],
            &["crop_ratio"],
            &[
                "gradient_custom",
                "gradient_use_custom",
                "gradient_selected_stop",
                "gradient_selected_preset",
                "gradient_shape",
                "gradient_blend",
                "gradient_reverse",
            ],
            &["effects/last"],
            &["effects/values/0"],
            &["effects/values/1"],
            &["effects/values/2"],
            &["effects/values/3"],
            &["effects/values/4"],
            &["effects/values/5"],
            &["effects/values/6"],
            &["effects/values/7"],
            &["effects/values/8"],
            &["effects/values/9"],
            &["effects/values/10"],
        ]
    }

    fn reconcile_patch(&mut self) {
        if self.oval_circle {
            self.selection_ratio = Some(crate::constraints::AspectRatio::SQUARE);
            self.selection_ratio_value = crate::constraints::AspectRatio::SQUARE;
            self.oval_circle = false;
        }
        self.effects.reconcile_last();
    }

    fn validate(&self) -> Result<()> {
        let ratio_valid = |ratio: crate::constraints::AspectRatio| {
            (1..=16384).contains(&ratio.width) && (1..=16384).contains(&ratio.height)
        };
        ensure!(self.brushes.valid(), "Invalid brush settings.");
        ensure!(self.text_defaults.valid(), "Invalid text defaults.");
        ensure!(
            self.effects.valid(),
            "Invalid remembered effect parameters."
        );
        // Rounded selections may retain geometry outside the canvas. Their u32
        // radius is clamped to each shape when used, rather than to canvas size.
        ensure!(
            self.selection_feather_radius <= 128,
            "Invalid feather radius."
        );
        ensure!(
            ratio_valid(self.selection_ratio_value)
                && ratio_valid(self.crop_ratio_value)
                && self.selection_ratio.is_none_or(ratio_valid)
                && self.crop_ratio.is_none_or(ratio_valid),
            "Invalid aspect ratio."
        );
        ensure!(
            self.wand_match.gap_radius <= crate::selection::MAX_GAP_RADIUS
                && self.fill_match.gap_radius <= crate::selection::MAX_GAP_RADIUS,
            "Invalid gap radius."
        );
        ensure!(
            (1..=4096).contains(&self.grid_spacing),
            "Invalid grid spacing."
        );
        ensure!(
            self.toolbox_width.is_finite() && (140.0..=400.0).contains(&self.toolbox_width),
            "Invalid toolbox width."
        );
        ensure!(
            (1..=100).contains(&self.jpeg_quality),
            "Invalid JPEG quality."
        );
        ensure!(
            (1..=1200).contains(&self.pdf_export_dpi)
                && (1..=1200).contains(&self.pdf_preferences.dpi),
            "Invalid PDF resolution."
        );
        ensure!(
            self.gradient_selected_stop
                < self
                    .gradient_custom
                    .map_or(crate::gradient::MAX_STOPS, |ramp| ramp.stops().len()),
            "Invalid gradient stop index."
        );
        ensure!(
            !self.gradient_use_custom || self.gradient_custom.is_some(),
            "Missing custom gradient."
        );
        Ok(())
    }
}

#[derive(Default)]
pub(super) struct Storage {
    path: Option<PathBuf>,
    loading: bool,
    saving: bool,
    timer: bool,
    token: u64,
    observed: Preferences,
    pending: Patch,
    error_reported: bool,
    exit_after_save: Option<bool>,
    exit_save_failed: bool,
}

impl Storage {
    pub(super) fn is_loading(&self) -> bool {
        self.loading
    }

    pub(super) fn defer_exit(&mut self, quit: bool) -> bool {
        if !self.loading
            && !self.saving
            && !self.timer
            && (self.path.is_none() || self.pending.is_empty())
        {
            return false;
        }
        self.exit_after_save = Some(quit || self.exit_after_save.unwrap_or(false));
        true
    }

    fn take_write(&mut self) -> Option<Patch> {
        if self.loading || self.saving || self.path.is_none() || self.pending.is_empty() {
            return None;
        }
        self.saving = true;
        self.exit_save_failed = false;
        Some(std::mem::take(&mut self.pending))
    }

    fn take_failed_exit(&mut self) -> bool {
        std::mem::take(&mut self.exit_save_failed)
    }

    fn failed_write(&mut self, mut patch: Patch) {
        self.saving = false;
        patch.merge(&self.pending);
        self.pending = patch;
        self.exit_save_failed = self.exit_after_save.take().is_some();
    }
}

impl Workspace {
    pub(super) fn load_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_store.path = settings::default_path();
        self.settings_store.observed = self.preferences.clone();
        let Some(path) = self.settings_store.path.clone() else {
            self.errors.push("The settings location is unavailable. Preferences will only be remembered for this session.".into());
            return;
        };
        self.settings_store.loading = true;
        let job = cx
            .background_executor()
            .spawn(async move { settings::load::<Preferences>(&path) });
        cx.spawn_in(window, async move |workspace, cx| {
            let result = job.await;
            let _ = workspace.update_in(cx, |this, window, cx| {
                this.record_settings();
                this.settings_store.loading = false;
                match result.and_then(|mut snapshot| {
                    this.settings_store.pending.apply(&mut snapshot)?;
                    let confirmed = snapshot.preferences.clone();
                    let mut live = Patch::default();
                    live.record(&this.settings_store.observed, &this.preferences)?;
                    live.apply(&mut snapshot)?;
                    Ok((snapshot, confirmed))
                }) {
                    Ok((snapshot, mut confirmed)) => {
                        let tool = this.preferences.tool;
                        // A startup read must not switch tools underneath an active
                        // draft/gesture, even if that gesture used the default tool.
                        let keep_tool = this.form.is_some() || this.pending_edit_name().is_some()
                            || this.numeric_edit.is_some() || this.toolbox_drag.is_some() || this.pan_drag.is_some();
                        this.preferences = snapshot.preferences;
                        if keep_tool {
                            this.preferences.tool = tool;
                            confirmed.tool = tool;
                            this.settings_store.pending.preferences.insert("tool".into(), serde_json::to_value(tool).unwrap());
                        }
                        this.effect_memory.restore(&this.preferences.effects);
                        this.preferences.effects = this.effect_memory.preferences();
                        this.settings_store.observed = confirmed;
                        this.color_memory = snapshot.colors;
                        this.gradient_revision = this.gradient_revision.wrapping_add(1);
                        this.reconcile_gradient_preset();
                        this.refresh_gradient(window, cx);
                        if !keep_tool && tool != this.preferences.tool {
                            this.begin_layer_transform_preview(window, cx);
                        }
                        this.start_settings_write(window, cx);
                        this.finish_settings_exit(window, cx);
                    }
                    Err(error) => {
                        this.settings_store.error_reported = true;
                        this.settings_store.exit_after_save = None;
                        this.errors.push(format!("Could not load settings: {error:#}. Current preferences remain available for this session."));
                    }
                }
                cx.notify();
            });
        }).detach();
    }

    fn record_settings(&mut self) -> bool {
        let committed = self.preferences.committed(
            &self.settings_store.observed,
            self.numeric_edit.as_ref().map(|edit| edit.field),
            self.toolbox_drag.is_some(),
        );
        if self.settings_store.observed == committed {
            return false;
        }
        match self
            .settings_store
            .pending
            .record(&self.settings_store.observed, &committed)
        {
            Ok(()) => {
                self.settings_store.observed = committed;
                self.settings_store.exit_save_failed = false;
                true
            }
            Err(error) => {
                if !self.settings_store.error_reported {
                    self.errors
                        .push(format!("Could not remember settings: {error:#}"));
                    self.settings_store.error_reported = true;
                }
                false
            }
        }
    }

    pub(super) fn sync_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.record_settings() {
            self.schedule_settings_write(window, cx);
        }
    }

    fn schedule_settings_write(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.settings_store.loading || self.settings_store.saving || self.settings_store.timer {
            return;
        }
        self.settings_store.timer = true;
        self.settings_store.token = self.settings_store.token.wrapping_add(1);
        let token = self.settings_store.token;
        cx.spawn_in(window, async move |workspace, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(250))
                .await;
            let _ = workspace.update_in(cx, |this, window, cx| {
                if !this.settings_store.timer || this.settings_store.token != token {
                    return;
                }
                this.settings_store.timer = false;
                this.record_settings();
                this.start_settings_write(window, cx);
                this.finish_settings_exit(window, cx);
            });
        })
        .detach();
    }

    pub(super) fn flush_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.sync_settings(window, cx);
        if self.settings_store.timer {
            self.settings_store.timer = false;
            self.settings_store.token = self.settings_store.token.wrapping_add(1);
        }
        // A previous failure leaves pending values without an armed timer.
        self.start_settings_write(window, cx);
    }

    pub(super) fn queue_settings_patch(&mut self, patch: &Patch) {
        self.settings_store.pending.merge(patch);
        self.settings_store.exit_save_failed = false;
    }

    pub(super) fn defer_settings_exit(
        &mut self,
        quit: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.record_settings();
        // After a failed exit-time retry, a second explicit Close/Quit may
        // proceed. A permanently unwritable settings file must not trap users.
        if self.settings_store.take_failed_exit() {
            return false;
        }
        self.flush_settings(window, cx);
        self.settings_store.defer_exit(quit)
    }

    pub(super) fn change_color_memory(
        &mut self,
        changes: Changes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.color_memory.apply(&changes);
        self.settings_store.exit_save_failed = false;
        self.settings_store.pending.colors.merge(&changes);
        self.record_settings();
        self.schedule_settings_write(window, cx);
        cx.notify();
    }

    pub(super) fn set_color(
        &mut self,
        swatch: usize,
        color: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_color_memory(Changes::swatch(swatch, color), window, cx);
    }

    pub(super) fn remember_color(
        &mut self,
        color: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_color_memory(Changes::color(color), window, cx);
    }

    fn start_settings_write(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.settings_store.path.clone() else {
            return;
        };
        let Some(patch) = self.settings_store.take_write() else {
            return;
        };
        let saved = patch.clone();
        let job = cx
            .background_executor()
            .spawn(async move { settings::save::<Preferences>(&path, &saved) });
        cx.spawn_in(window, async move |workspace, cx| {
            let result = job.await;
            let _ = workspace.update_in(cx, |this, window, cx| {
                this.settings_store.saving = false;
                this.record_settings();
                match result {
                    Ok(mut snapshot) => {
                        // Never change the active tool or its live preferences on
                        // save completion. Other instances' untouched keys stay on disk.
                        snapshot.colors.apply(&this.settings_store.pending.colors);
                        let colors_changed = this.color_memory != snapshot.colors;
                        this.color_memory = snapshot.colors;
                        this.settings_store.error_reported = false;
                        this.settings_store.exit_save_failed = false;
                        if colors_changed { this.refresh_gradient(window, cx); }
                        this.start_settings_write(window, cx);
                        this.finish_settings_exit(window, cx);
                    }
                    Err(error) => {
                        this.settings_store.failed_write(patch);
                        if !this.settings_store.error_reported || this.settings_store.exit_save_failed {
                            let recovery = if this.settings_store.exit_save_failed {
                                "Close or quit again to exit without saving preferences, or change/reset a setting to retry."
                            } else {
                                "Change or reset a setting, or close the app, to retry saving."
                            };
                            this.errors.push(format!("Could not save settings: {error:#}. Current preferences remain available. {recovery}"));
                            this.settings_store.error_reported = true;
                        }
                    }
                }
                cx.notify();
            });
        }).detach();
    }

    fn finish_settings_exit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.settings_store.loading || self.settings_store.saving || self.settings_store.timer {
            return;
        }
        if let Some(quit) = self.settings_store.exit_after_save.take()
            && self.request_exit(quit, window, cx)
        {
            if quit {
                cx.quit();
            } else {
                window.remove_window();
            }
        }
    }

    pub(super) fn reconcile_gradient_preset(&mut self) {
        if self.gradient_store.loading {
            return;
        }
        self.preferences.gradient_selected_preset =
            self.preferences.gradient_selected_preset.filter(|id| {
                self.gradient_library
                    .get(*id)
                    .is_some_and(|preset| preset.value == self.current_gradient_preset())
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn high_resolution_text_defaults_reload_at_the_new_limit() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        for size in [300.0, crate::text::MAX_PIXEL_SIZE] {
            let mut preferences = serde_json::to_value(Preferences::default()).unwrap();
            preferences["text_defaults"]["size"] = json!(size);
            let value = json!({"version":1,"preferences":preferences});
            std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
            let loaded = settings::load::<Preferences>(&path).unwrap();
            let loaded = serde_json::to_value(loaded.preferences).unwrap();
            assert_eq!(loaded["text_defaults"]["size"], json!(size));
        }
    }

    #[test]
    fn malformed_unknown_and_out_of_range_settings_are_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let base = json!({"version":1,"preferences":serde_json::to_value(Preferences::default()).unwrap()});
        let mut bad_files = Vec::new();
        for (field, value) in [
            ("tool", json!("Unknown")),
            ("grid_spacing", json!(0)),
            ("selection_feather_radius", json!(129)),
            ("toolbox_width", json!(99999)),
            ("pdf_export_dpi", json!(1201)),
            ("newer_field", json!(true)),
            ("gradient_use_custom", json!(true)),
        ] {
            let mut bad = base.clone();
            bad["preferences"][field] = value;
            bad_files.push(bad);
        }
        let mut bad = base.clone();
        bad["preferences"]["brushes"]["brush"]["diameter"] = json!(0);
        bad_files.push(bad);
        let mut bad = base.clone();
        bad["preferences"]["text_defaults"]["size"] = json!(crate::text::MAX_PIXEL_SIZE + 1.0);
        bad_files.push(bad);
        let mut bad = base.clone();
        bad["version"] = json!(2);
        bad_files.push(bad);
        let mut bad = base.clone();
        bad["colors"] = json!({"swatches":[0,16777215],"recent":[1,1]});
        bad_files.push(bad);
        let mut bad = base.clone();
        let mut curve =
            serde_json::to_value(crate::adjustments::Adjustment::Curves(Default::default()))
                .unwrap();
        curve["Curves"]["red"]["count"] = json!(255);
        bad["preferences"]["effects"]["values"][6] = json!({"Adjustment":curve});
        bad_files.push(bad);
        let patch = Patch {
            colors: Changes::color(7),
            ..Default::default()
        };
        for bad in bad_files {
            let bytes = serde_json::to_vec(&bad).unwrap();
            std::fs::write(&path, &bytes).unwrap();
            assert!(settings::load::<Preferences>(&path).is_err());
            assert!(settings::save::<Preferences>(&path, &patch).is_err());
            assert_eq!(std::fs::read(&path).unwrap(), bytes);
        }
        for bytes in [b"broken".to_vec(), vec![b' '; 128 * 1024 + 1]] {
            std::fs::write(&path, &bytes).unwrap();
            assert!(settings::save::<Preferences>(&path, &patch).is_err());
            assert_eq!(std::fs::read(&path).unwrap(), bytes);
        }
    }

    #[test]
    fn lock_failures_preserve_disk_settings_and_a_retry_keeps_queued_changes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut first = Patch {
            colors: Changes::color(0x123456),
            ..Default::default()
        };
        settings::save::<Preferences>(&path, &first).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let lock = std::fs::File::options()
            .read(true)
            .write(true)
            .open(path.with_extension("lock"))
            .unwrap();
        lock.lock().unwrap();
        first.preferences.insert("grid_spacing".into(), json!(33));
        assert!(settings::save::<Preferences>(&path, &first).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        let newer = Patch {
            colors: Changes::swatch(0, 0xabcdef),
            ..Default::default()
        };
        first.merge(&newer);
        drop(lock);
        let result = settings::save::<Preferences>(&path, &first).unwrap();
        assert_eq!(result.preferences.grid_spacing, 33);
        assert_eq!(result.colors.swatches[0], 0xabcdef);
        assert_eq!(result.colors.recent, [0xabcdef, 0x123456]);
    }
}
