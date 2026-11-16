use std::sync::{Arc, Mutex};

use image::{Rgba, RgbaImage};

use crate::document::{MAX_IMAGE_BYTES, MAX_NEW_EDGE};

pub mod path;

pub const MAX_TEXT_CHARS: usize = 4_000;
pub const MAX_FAMILY_CHARS: usize = 128;
pub const MIN_PIXEL_SIZE: f32 = 1.0;
pub const MAX_PIXEL_SIZE: f32 = 4096.0;
// Letter spacing is extra advance in canvas pixels after each glyph.
pub const MIN_LETTER_SPACING: f32 = -64.0;
pub const MAX_LETTER_SPACING: f32 = 256.0;
// Line spacing is the distance between baselines as a multiple of the size.
pub const MIN_LINE_SPACING: f32 = 0.5;
pub const MAX_LINE_SPACING: f32 = 4.0;
pub const DEFAULT_LINE_SPACING: f32 = 1.2;

// GPUI can lay out a line, but rasterize_glyph is crate-private. cosmic-text
// shapes with HarfRust and returns coverage through SwashCache.
static FONTS: Mutex<Option<(cosmic_text::FontSystem, cosmic_text::SwashCache)>> = Mutex::new(None);

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Spacing {
    pub letter: f32,
    pub line: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            letter: 0.0,
            line: DEFAULT_LINE_SPACING,
        }
    }
}

impl Spacing {
    pub fn allowed(self) -> bool {
        self.letter.is_finite()
            && self.line.is_finite()
            && (MIN_LETTER_SPACING..=MAX_LETTER_SPACING).contains(&self.letter)
            && (MIN_LINE_SPACING..=MAX_LINE_SPACING).contains(&self.line)
    }
}

/// Face choices shared by rasterization, caret layout, and cached text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FontFace {
    pub bold: bool,
    pub italic: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TextLayout {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub align: TextAlign,
}
impl TextLayout {
    pub fn is_auto(&self) -> bool {
        *self == Self::default()
    }
    pub fn allowed(self) -> bool {
        [self.width, self.height]
            .into_iter()
            .flatten()
            .all(|edge| (1..=MAX_NEW_EDGE).contains(&edge))
            && (self.height.is_none() || self.width.is_some())
    }
}

/// Everything that decides how a text layer rasterizes.
#[derive(Clone, Debug, PartialEq)]
pub struct TextSpec {
    pub source: String,
    pub family: String,
    pub face: FontFace,
    pub size: f32,
    pub color: Rgba<u8>,
    pub spacing: Spacing,
    pub layout: TextLayout,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub source: String,
    pub family: String,
    pub face: FontFace,
    pub size: f32,
    pub color: Rgba<u8>,
    pub spacing: Spacing,
    pub layout: TextLayout,
    pub resolved_family: String,
    // Layout origin inside the bitmap. The canvas anchor is layer offset plus this.
    pub origin: (i32, i32),
}

impl TextStyle {
    pub fn substituted(&self) -> bool {
        !is_generic(&self.family) && !self.family.eq_ignore_ascii_case(&self.resolved_family)
    }

    pub fn anchor(&self, offset: (i32, i32)) -> (i32, i32) {
        (
            offset.0.saturating_add(self.origin.0),
            offset.1.saturating_add(self.origin.1),
        )
    }

    pub fn offset_for(&self, anchor: (i32, i32)) -> (i32, i32) {
        (
            anchor.0.saturating_sub(self.origin.0),
            anchor.1.saturating_sub(self.origin.1),
        )
    }
}

#[derive(Clone, Debug)]
pub struct RenderedText {
    pub image: Arc<RgbaImage>,
    pub style: TextStyle,
}

impl RenderedText {
    pub fn matches(&self, spec: &TextSpec) -> bool {
        self.style.source == spec.source
            && self.style.family == spec.family
            && self.style.face == spec.face
            && self.style.size == spec.size
            && self.style.color == spec.color
            && self.style.spacing == spec.spacing
            && self.style.layout == spec.layout
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextError {
    TooLong,
    Family,
    Size,
    Spacing,
    TooLarge,
}

impl TextError {
    pub fn message(self) -> &'static str {
        match self {
            Self::TooLong => "Text is limited to 4000 characters.",
            Self::Family => "The font name is too long.",
            Self::Size => "Text size must be between 1 and 4096 pixels.",
            Self::Spacing => "Text spacing is out of range.",
            Self::TooLarge => "The text is too large to rasterize.",
        }
    }
}

pub fn is_generic(family: &str) -> bool {
    matches!(
        family.trim().to_ascii_lowercase().as_str(),
        "sans-serif" | "sans" | "serif" | "monospace" | "mono"
    )
}

pub fn normalize_family(family: &str) -> String {
    match family.trim().to_ascii_lowercase().as_str() {
        "sans-serif" | "sans" => "sans-serif".to_owned(),
        "serif" => "serif".to_owned(),
        "monospace" | "mono" => "monospace".to_owned(),
        _ => family.trim().to_owned(),
    }
}

pub fn fields_allowed(
    source: &str,
    family: &str,
    resolved: &str,
    size: f32,
    spacing: Spacing,
) -> bool {
    spacing.allowed()
        && source.chars().count() <= MAX_TEXT_CHARS
        && source
            .chars()
            .all(|ch| ch == '\n' || ch == '\t' || !ch.is_control())
        && family.chars().count() <= MAX_FAMILY_CHARS
        && resolved.chars().count() <= MAX_FAMILY_CHARS
        && family.chars().all(|ch| !ch.is_control())
        && resolved.chars().all(|ch| !ch.is_control())
        && size.is_finite()
        && (MIN_PIXEL_SIZE..=MAX_PIXEL_SIZE).contains(&size)
}

#[cfg(test)]
pub fn render(
    source: &str,
    family: &str,
    pixel_size: f32,
    color: Rgba<u8>,
) -> Result<RenderedText, TextError> {
    render_spec(&TextSpec {
        layout: Default::default(),
        source: source.to_owned(),
        family: family.to_owned(),
        face: FontFace::default(),
        size: pixel_size,
        color,
        spacing: Spacing::default(),
    })
}

pub fn render_spec(spec: &TextSpec) -> Result<RenderedText, TextError> {
    let (source, pixel_size, color, spacing) =
        (spec.source.as_str(), spec.size, spec.color, spec.spacing);
    if !spacing.allowed() || !spec.layout.allowed() {
        return Err(TextError::Spacing);
    }
    if !pixel_size.is_finite() || !(MIN_PIXEL_SIZE..=MAX_PIXEL_SIZE).contains(&pixel_size) {
        return Err(TextError::Size);
    }
    if source.chars().count() > MAX_TEXT_CHARS
        || source
            .chars()
            .any(|ch| ch != '\n' && ch != '\t' && ch.is_control())
    {
        return Err(TextError::TooLong);
    }
    let family = normalize_family(&spec.family);
    if family.chars().count() > MAX_FAMILY_CHARS || family.chars().any(|ch| ch.is_control()) {
        return Err(TextError::Family);
    }
    with_fonts(|font_system, cache| {
        if source.is_empty() && spec.layout.width.is_none() {
            let (resolved, _) = resolve_family(font_system, &family, None);
            let mut result = rendered(
                RgbaImage::new(1, 1),
                source,
                family,
                pixel_size,
                color,
                spacing,
                spec.face,
                resolved,
                (0, 0),
            );
            result.style.layout = spec.layout;
            return Ok(result);
        }
        let mut buffer = layout(
            font_system,
            source,
            &family,
            pixel_size,
            spacing,
            spec.face,
            spec.layout,
        );

        let mut line_right = 0i32;
        let mut line_bottom = 0i32;
        let mut used_font = None;
        for run in buffer.layout_runs() {
            if !run.line_w.is_finite()
                || !run.line_top.is_finite()
                || !run.line_height.is_finite()
                || run.line_w > MAX_NEW_EDGE as f32
                || (spec.layout.height.is_none()
                    && run.line_top + run.line_height > MAX_NEW_EDGE as f32)
            {
                return Err(TextError::TooLarge);
            }
            line_right = line_right.max(span_end(run.line_w));
            line_bottom = line_bottom.max(span_end(run.line_top + run.line_height));
            if used_font.is_none() {
                used_font = run
                    .glyphs
                    .iter()
                    .find(|glyph| glyph.glyph_id != 0)
                    .map(|glyph| glyph.font_id);
            }
        }

        // Bound glyph rasterization to visible box lines. Caret layout remains
        // complete in caret_map, so overflowing source stays editable.
        if spec.layout.height.is_some() {
            buffer.set_size(
                spec.layout.width.map(|width| width as f32),
                spec.layout.height.map(|height| height as f32),
            );
        }
        let mut left = 0i32;
        let mut top = 0i32;
        let mut right = line_right;
        let mut bottom = line_bottom;
        let mut too_big = false;
        let ink = cosmic_text::Color::rgba(color.0[0], color.0[1], color.0[2], 255);
        buffer.draw(font_system, cache, ink, |x, y, w, h, pixel| {
            if too_big || pixel.a() == 0 || w == 0 || h == 0 {
                return;
            }
            let far_x = x.saturating_add(i32::try_from(w).unwrap_or(i32::MAX) - 1);
            let far_y = y.saturating_add(i32::try_from(h).unwrap_or(i32::MAX) - 1);
            left = left.min(x.min(0));
            top = top.min(y.min(0));
            right = right.max(far_x);
            bottom = bottom.max(far_y);
            if right.saturating_sub(left) >= MAX_NEW_EDGE as i32
                || bottom.saturating_sub(top) >= MAX_NEW_EDGE as i32
            {
                too_big = true;
            }
        });
        if too_big && spec.layout.width.is_none() {
            return Err(TextError::TooLarge);
        }
        if let Some(width) = spec.layout.width {
            left = 0;
            top = 0;
            right = width as i32 - 1;
            if let Some(height) = spec.layout.height {
                bottom = height as i32 - 1;
            }
        }
        let width = u32::try_from(right - left + 1).unwrap_or(u32::MAX).max(1);
        let height = u32::try_from(bottom - top + 1).unwrap_or(u32::MAX).max(1);
        if !image_allowed(width, height) {
            return Err(TextError::TooLarge);
        }
        let mut image = RgbaImage::new(width, height);
        buffer.draw(font_system, cache, ink, |x, y, w, h, pixel| {
            let coverage = pixel.a();
            if coverage == 0 || w == 0 || h == 0 {
                return;
            }
            let rgb = [pixel.r(), pixel.g(), pixel.b()];
            let columns = i32::try_from(w).unwrap_or(i32::MAX);
            let rows = i32::try_from(h).unwrap_or(i32::MAX);
            for row in 0..rows {
                for column in 0..columns {
                    let px = x.saturating_add(column).saturating_sub(left);
                    let py = y.saturating_add(row).saturating_sub(top);
                    if px < 0 || py < 0 {
                        continue;
                    }
                    let (Ok(px), Ok(py)) = (u32::try_from(px), u32::try_from(py)) else {
                        continue;
                    };
                    if px >= width || py >= height {
                        continue;
                    }
                    cover(image.get_pixel_mut(px, py), rgb, coverage, color.0[3]);
                }
            }
        });
        let (resolved, _) = resolve_family(font_system, &family, used_font);
        if resolved.chars().count() > MAX_FAMILY_CHARS {
            return Err(TextError::Family);
        }
        let mut result = rendered(
            image,
            source,
            family,
            pixel_size,
            color,
            spacing,
            spec.face,
            resolved,
            (-left, -top),
        );
        result.style.layout = spec.layout;
        Ok(result)
    })
}

fn layout(
    font_system: &mut cosmic_text::FontSystem,
    source: &str,
    family: &str,
    pixel_size: f32,
    spacing: Spacing,
    face: FontFace,
    text_layout: TextLayout,
) -> cosmic_text::Buffer {
    let metrics = cosmic_text::Metrics::new(pixel_size, line_height(pixel_size, spacing));
    let mut buffer = cosmic_text::Buffer::new(font_system, metrics);
    buffer.set_wrap(if text_layout.width.is_some() {
        cosmic_text::Wrap::WordOrGlyph
    } else {
        cosmic_text::Wrap::None
    });
    buffer.set_size(text_layout.width.map(|value| value as f32), None);
    let attrs = cosmic_text::Attrs::new()
        .family(family_attr(family))
        .weight(if face.bold {
            cosmic_text::Weight::BOLD
        } else {
            cosmic_text::Weight::NORMAL
        })
        .style(if face.italic {
            cosmic_text::Style::Italic
        } else {
            cosmic_text::Style::Normal
        })
        .letter_spacing(spacing.letter / pixel_size);
    buffer.set_text(source, &attrs, cosmic_text::Shaping::Advanced, None);
    for line in &mut buffer.lines {
        line.set_align(Some(match text_layout.align {
            TextAlign::Left => cosmic_text::Align::Left,
            TextAlign::Center => cosmic_text::Align::Center,
            TextAlign::Right => cosmic_text::Align::Right,
        }));
    }
    buffer.shape_until_scroll(font_system, false);
    buffer
}

fn line_height(pixel_size: f32, spacing: Spacing) -> f32 {
    (pixel_size * spacing.line).max(1.0)
}

/// Which visual side of a shared wrap boundary owns the caret.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CaretAffinity {
    Upstream,
    #[default]
    Downstream,
}

/// A caret position in layout coordinates, where the text anchor is the origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaretStop {
    pub byte: usize,
    pub line: usize,
    pub x: f32,
    pub top: f32,
}

/// Caret positions for every grapheme boundary of one layout.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CaretMap {
    pub spec: Option<TextSpec>,
    pub stops: Vec<CaretStop>,
    pub line_height: f32,
}

impl CaretMap {
    pub fn selection_rects(&self, range: std::ops::Range<usize>) -> Vec<(f32, f32, f32, f32)> {
        let mut rectangles = Vec::new();
        for pair in self.stops.windows(2) {
            let (start, end) = (pair[0], pair[1]);
            if start.byte == end.byte || start.byte < range.start || start.byte >= range.end {
                continue;
            }
            let width = if start.line == end.line {
                (end.x - start.x).abs()
            } else {
                self.line_height * 0.35
            };
            if width > 0. {
                rectangles.push((
                    if start.line == end.line {
                        start.x.min(end.x)
                    } else {
                        start.x
                    },
                    start.top,
                    width,
                    self.line_height,
                ));
            }
        }
        rectangles.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
        let mut merged: Vec<(f32, f32, f32, f32)> = Vec::new();
        for (x, top, width, height) in rectangles {
            if let Some(last) = merged.last_mut()
                && last.1 == top
                && x <= last.0 + last.2 + 0.1
            {
                last.2 = (x + width).max(last.0 + last.2) - last.0;
            } else {
                merged.push((x, top, width, height));
            }
        }
        merged
    }

    pub fn matches(&self, spec: &TextSpec) -> bool {
        self.spec.as_ref().is_some_and(|own| {
            own.source == spec.source
                && own.family == spec.family
                && own.face == spec.face
                && own.size == spec.size
                && own.spacing == spec.spacing
                && own.layout == spec.layout
        })
    }

    /// The stop at a byte offset, or the nearest earlier one.
    pub fn at(&self, byte: usize) -> Option<CaretStop> {
        let index = self.stops.partition_point(|stop| stop.byte <= byte);
        index.checked_sub(1).map(|index| self.stops[index])
    }

    pub fn at_affinity(&self, byte: usize, affinity: CaretAffinity) -> Option<CaretStop> {
        if affinity == CaretAffinity::Upstream {
            let first = self.stops.partition_point(|stop| stop.byte < byte);
            if let Some(stop) = self.stops.get(first).filter(|stop| stop.byte == byte) {
                return Some(*stop);
            }
        }
        self.at(byte)
    }

    fn position(&self, stop: CaretStop) -> (usize, CaretAffinity) {
        let affinity = if self
            .at(stop.byte)
            .is_some_and(|last| last.line != stop.line)
        {
            CaretAffinity::Upstream
        } else {
            CaretAffinity::Downstream
        };
        (stop.byte, affinity)
    }

    pub fn hit_position(&self, x: f32, y: f32) -> Option<(usize, CaretAffinity)> {
        let last = self.stops.last()?;
        let line = if self.line_height > 0. && y.is_finite() {
            ((y / self.line_height).floor().max(0.) as usize).min(last.line)
        } else {
            0
        };
        self.stops
            .iter()
            .filter(|stop| stop.line == line)
            .min_by(|a, b| (a.x - x).abs().total_cmp(&(b.x - x).abs()))
            .map(|stop| self.position(*stop))
    }

    pub fn line_edge_position(
        &self,
        byte: usize,
        affinity: CaretAffinity,
        end: bool,
    ) -> Option<(usize, CaretAffinity)> {
        let current = self.at_affinity(byte, affinity)?;
        let stops = self.stops.iter().filter(|stop| stop.line == current.line);
        let stop = if end {
            stops.max_by(|a, b| a.x.total_cmp(&b.x))
        } else {
            stops.min_by(|a, b| a.x.total_cmp(&b.x))
        }?;
        Some(self.position(*stop))
    }

    pub fn vertical_position(
        &self,
        byte: usize,
        affinity: CaretAffinity,
        down: bool,
    ) -> Option<(usize, CaretAffinity)> {
        let current = self.at_affinity(byte, affinity)?;
        let line = if down {
            current.line.checked_add(1)?
        } else {
            current.line.checked_sub(1)?
        };
        self.stops
            .iter()
            .filter(|stop| stop.line == line)
            .min_by(|a, b| (a.x - current.x).abs().total_cmp(&(b.x - current.x).abs()))
            .map(|stop| self.position(*stop))
    }
}

pub fn caret_map(spec: &TextSpec) -> Result<CaretMap, TextError> {
    if !spec.size.is_finite() || !(MIN_PIXEL_SIZE..=MAX_PIXEL_SIZE).contains(&spec.size) {
        return Err(TextError::Size);
    }
    if !spec.spacing.allowed() || !spec.layout.allowed() {
        return Err(TextError::Spacing);
    }
    let family = normalize_family(&spec.family);
    let line_height = line_height(spec.size, spec.spacing);
    let mut stops = Vec::with_capacity(spec.source.len() + 1);
    with_fonts(|font_system, _| {
        let buffer = layout(
            font_system,
            &spec.source,
            &family,
            spec.size,
            spec.spacing,
            spec.face,
            spec.layout,
        );
        let mut start = 0;
        for (line, text) in spec.source.split('\n').enumerate() {
            for (index, _) in
                unicode_segmentation::UnicodeSegmentation::grapheme_indices(text, true)
                    .chain(std::iter::once((text.len(), "")))
            {
                let cursor = cosmic_text::Cursor::new(line, index);
                let mut positioned = false;
                for (visual, run) in buffer
                    .layout_runs()
                    .enumerate()
                    .filter(|(_, run)| run.line_i == line)
                {
                    if let Some(mut x) = run.cursor_position(&cursor) {
                        if run.glyphs.is_empty() {
                            x = match spec.layout.align {
                                TextAlign::Left => 0.,
                                TextAlign::Center => spec.layout.width.unwrap_or(0) as f32 / 2.,
                                TextAlign::Right => spec.layout.width.unwrap_or(0) as f32,
                            };
                        }
                        stops.push(CaretStop {
                            byte: start + index,
                            line: visual,
                            x: if x.is_finite() { x } else { 0. },
                            top: if run.line_top.is_finite() {
                                run.line_top
                            } else {
                                0.
                            },
                        });
                        positioned = true;
                    }
                }
                if !positioned {
                    stops.push(CaretStop {
                        byte: start + index,
                        line,
                        x: 0.,
                        top: line as f32 * line_height,
                    });
                }
            }
            start += text.len() + 1;
        }
    });
    Ok(CaretMap {
        spec: Some(spec.clone()),
        stops,
        line_height,
    })
}

/// Installed font families, sorted for a picker.
pub struct FontFamilies {
    pub names: Vec<String>,
    pub generic_previews: [String; 3],
}

pub fn families() -> FontFamilies {
    let (mut names, generic_previews) = with_fonts(|font_system, _| {
        let names = font_system
            .db()
            .faces()
            .filter_map(|face| face.families.first().map(|(name, _)| name.clone()))
            .filter(|name| {
                !name.starts_with('.')
                    && name.chars().count() <= MAX_FAMILY_CHARS
                    && !name.chars().any(char::is_control)
            })
            .collect::<Vec<_>>();
        // GPUI expects an installed family name rather than CSS generic names.
        let previews = ["sans-serif", "serif", "monospace"].map(|family| {
            let id = font_system.db().query(&cosmic_text::fontdb::Query {
                families: &[family_attr(family)],
                ..Default::default()
            });
            id.and_then(|id| face_family(font_system, id))
                .unwrap_or_else(|| family.to_owned())
        });
        (names, previews)
    });
    names.sort_by_key(|name| name.to_lowercase());
    names.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    FontFamilies {
        names,
        generic_previews,
    }
}

#[allow(clippy::too_many_arguments)]
fn rendered(
    image: RgbaImage,
    source: &str,
    family: String,
    pixel_size: f32,
    color: Rgba<u8>,
    spacing: Spacing,
    face: FontFace,
    resolved: String,
    origin: (i32, i32),
) -> RenderedText {
    RenderedText {
        image: Arc::new(image),
        style: TextStyle {
            layout: Default::default(),
            source: source.to_owned(),
            family,
            face,
            size: pixel_size,
            color,
            spacing,
            resolved_family: resolved,
            origin,
        },
    }
}

pub(crate) fn substitution(style: &TextStyle) -> Option<String> {
    if is_generic(&style.family) {
        return None;
    }
    with_fonts(|fonts, _| {
        if canonical_family(fonts, &style.family).is_some() {
            style
                .substituted()
                .then(|| format!("Cached text uses {}.", style.resolved_family))
        } else {
            Some(format!(
                "Using sans-serif instead of {} when edited.",
                style.family
            ))
        }
    })
}

fn with_fonts<T>(
    draw: impl FnOnce(&mut cosmic_text::FontSystem, &mut cosmic_text::SwashCache) -> T,
) -> T {
    let mut fonts = FONTS.lock().unwrap_or_else(|poison| poison.into_inner());
    if fonts.is_none() {
        *fonts = Some((
            cosmic_text::FontSystem::new(),
            cosmic_text::SwashCache::new(),
        ));
    }
    let (font_system, cache) = fonts.as_mut().expect("the font system was just installed");
    draw(font_system, cache)
}

fn family_attr(family: &str) -> cosmic_text::fontdb::Family<'_> {
    match family {
        "sans-serif" => cosmic_text::fontdb::Family::SansSerif,
        "serif" => cosmic_text::fontdb::Family::Serif,
        "monospace" => cosmic_text::fontdb::Family::Monospace,
        _ => cosmic_text::fontdb::Family::Name(family),
    }
}

fn resolve_family(
    font_system: &mut cosmic_text::FontSystem,
    family: &str,
    used: Option<cosmic_text::fontdb::ID>,
) -> (String, bool) {
    if is_generic(family) {
        let resolved = used
            .and_then(|id| face_family(font_system, id))
            .unwrap_or_else(|| family.to_owned());
        return (resolved, false);
    }
    if let Some(canonical) = canonical_family(font_system, family) {
        return (canonical, false);
    }
    let resolved = used
        .and_then(|id| face_family(font_system, id))
        .unwrap_or_else(|| "sans-serif".to_owned());
    (resolved, true)
}

fn canonical_family(font_system: &mut cosmic_text::FontSystem, requested: &str) -> Option<String> {
    font_system.db_mut().faces().find_map(|face| {
        face.families
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(requested))
            .map(|(name, _)| name.clone())
    })
}

fn face_family(
    font_system: &mut cosmic_text::FontSystem,
    id: cosmic_text::fontdb::ID,
) -> Option<String> {
    font_system
        .db_mut()
        .face(id)
        .and_then(|face| face.families.first().map(|(name, _)| name.clone()))
}

fn span_end(length: f32) -> i32 {
    if !length.is_finite() || length <= 0.0 {
        return 0;
    }
    length.ceil() as i32 - 1
}

fn image_allowed(width: u32, height: u32) -> bool {
    width > 0
        && height > 0
        && width <= MAX_NEW_EDGE
        && height <= MAX_NEW_EDGE
        && u64::from(width)
            .saturating_mul(u64::from(height))
            .saturating_mul(4)
            <= MAX_IMAGE_BYTES
}

fn cover(pixel: &mut Rgba<u8>, rgb: [u8; 3], coverage: u8, opacity: u8) {
    let source_a = ((u32::from(coverage) * u32::from(opacity) + 127) / 255) as u8;
    if source_a == 0 {
        return;
    }
    let dest_a = pixel.0[3];
    let inverse = 255 - u32::from(source_a);
    let out_a = (u32::from(source_a) + (u32::from(dest_a) * inverse + 127) / 255).min(255) as u8;
    if out_a == 0 {
        *pixel = Rgba([0, 0, 0, 0]);
        return;
    }
    let channel = |source: u8, dest: u8| -> u8 {
        let blended = u32::from(source) * u32::from(source_a)
            + u32::from(dest) * u32::from(dest_a) * inverse / 255;
        ((blended + u32::from(out_a) / 2) / u32::from(out_a)) as u8
    };
    *pixel = Rgba([
        channel(rgb[0], pixel.0[0]),
        channel(rgb[1], pixel.0[1]),
        channel(rgb[2], pixel.0[2]),
        out_a,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn max_alpha(image: &RgbaImage) -> u8 {
        image.pixels().map(|pixel| pixel.0[3]).max().unwrap_or(0)
    }

    fn spec(source: &str, spacing: Spacing) -> TextSpec {
        TextSpec {
            layout: Default::default(),
            source: source.into(),
            family: "sans-serif".into(),
            face: FontFace::default(),
            size: 32.0,
            color: Rgba([0, 0, 0, 255]),
            spacing,
        }
    }

    #[test]
    fn high_resolution_text_renders_and_keeps_caret_and_metadata_sizes() {
        for size in [300.0, 1024.0, MAX_PIXEL_SIZE] {
            let mut spec = spec("A", Spacing::default());
            spec.size = size;
            // Exercise the maximum's validation/layout without a huge bitmap.
            if size == MAX_PIXEL_SIZE {
                spec.source.clear();
            }
            let rendered = render_spec(&spec).unwrap();
            assert_eq!(rendered.style.size, size);
            assert!(fields_allowed(
                &spec.source,
                &spec.family,
                &rendered.style.resolved_family,
                size,
                spec.spacing,
            ));
            assert!(caret_map(&spec).unwrap().matches(&spec));
            if !spec.source.is_empty() {
                assert!(rendered.image.height() > 256);
                assert!(max_alpha(&rendered.image) > 0);
            }
        }
        for size in [MAX_PIXEL_SIZE + 1.0, f32::INFINITY, f32::NAN] {
            let mut spec = spec("A", Spacing::default());
            spec.size = size;
            assert!(matches!(render_spec(&spec), Err(TextError::Size)));
            assert!(matches!(caret_map(&spec), Err(TextError::Size)));
            assert!(!fields_allowed(
                "A",
                "sans-serif",
                "sans-serif",
                size,
                spec.spacing
            ));
        }
    }
}
