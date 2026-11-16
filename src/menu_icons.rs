//! Selective menu imagery shares the toolbox's SVG glyphs on both platforms.

#[cfg(any(target_os = "macos", target_os = "windows", test))]
use anyhow::{Context as _, Result};
use gpui::Action;
#[cfg(any(target_os = "macos", target_os = "windows", test))]
use gpui::AssetSource;

use crate::commands::*;
#[cfg(any(target_os = "macos", target_os = "windows", test))]
use crate::icons::Icons;

pub(crate) fn for_action(action: &dyn Action) -> Option<&'static str> {
    let action = action.as_any();
    if let Some(SelectTool(tool)) = action.downcast_ref::<SelectTool>() {
        return Some(tool.info().icon);
    }
    if let Some(align) = action.downcast_ref::<AlignLayer>() {
        return Some(match align {
            AlignLayer::Top => "align-top",
            AlignLayer::Bottom => "align-bottom",
            AlignLayer::Left => "align-left",
            AlignLayer::Right => "align-right",
            AlignLayer::Horizontal => "align-h-center",
            AlignLayer::Both => "align-center",
            AlignLayer::Vertical => "align-v-center",
        });
    }
    if let Some(transform) = action.downcast_ref::<ImageAdjust>() {
        return Some(match transform {
            ImageAdjust::RotateLeft => "rotate",
            ImageAdjust::RotateRight => "rotate-right",
            ImageAdjust::RotateHalf => "rotate-half",
            ImageAdjust::FlipHorizontal => "flip",
            ImageAdjust::FlipVertical => "flip-vertical",
        });
    }
    if let Some(transform) = action.downcast_ref::<LayerAdjust>() {
        return Some(match transform {
            LayerAdjust::RotateLeft => "rotate",
            LayerAdjust::RotateRight => "rotate-right",
            LayerAdjust::RotateHalf => "rotate-half",
            LayerAdjust::FlipHorizontal => "flip",
            LayerAdjust::FlipVertical => "flip-vertical",
        });
    }
    macro_rules! icons {
        ($($kind:ty => $icon:literal),* $(,)?) => {
            $(if action.is::<$kind>() { return Some($icon); })*
        };
    }
    icons! {
        NewImage => "new", OpenImage => "folder", OpenAsLayers => "folder",
        SaveImage => "save", SaveImageAs => "save", SaveProjectAs => "save",
        ExportImage => "export",
        Undo => "undo", Redo => "redo", CutImage => "cut", CopyImage => "duplicate",
        PasteImage => "paste", NewFromClipboard => "paste",
        NewLayer => "plus", DuplicateLayer => "duplicate", DeleteLayer => "trash",
        MergeDown => "merge", RaiseLayer => "arrow-up", LowerLayer => "arrow-down",
        ToggleLayerLink => "link", DetachShadow => "unlink", AttachLayerAbove => "link",
        ToggleLayerVisibility => "eye", ToggleAlphaLock => "lock-alpha", TogglePositionLock => "lock-position", FitLayerToCanvas => "scale", ScaleImage => "scale",
        ApplyCrop => "crop", CropToSelection => "crop", CropImageToContent => "crop",
        CropLayerToContent => "crop", StrokePath => "stroke", StrokeSelection => "stroke", StrokeLayer => "stroke",
        SwapColors => "swap",
        RepeatLastEffect => "redo", ReopenLastEffect => "redo",
        ResetCurrentTool => "reset", ResetAllTools => "reset",
        ToggleRightSidebar => "sidebar-right-open",
    }
    None
}

/// Straight-alpha RGBA for native icons; geometry stays in the shared SVG source.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
pub(crate) fn rasterize(icon: &str, pixels: u32, color: u32) -> Result<Vec<u8>> {
    anyhow::ensure!((1..=256).contains(&pixels), "invalid menu icon size");
    let svg = Icons.load(icon)?.context("unknown menu icon")?;
    let tree = resvg::usvg::Tree::from_data(&svg, &resvg::usvg::Options::default())?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixels, pixels).context("invalid icon size")?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(pixels as f32 / 24., pixels as f32 / 24.),
        &mut pixmap.as_mut(),
    );
    let rgb = [(color >> 16) as u8, (color >> 8) as u8, color as u8];
    Ok(pixmap
        .pixels()
        .iter()
        .flat_map(|pixel| [rgb[0], rgb[1], rgb[2], pixel.alpha()])
        .collect())
}
