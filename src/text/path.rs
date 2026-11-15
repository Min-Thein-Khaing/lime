//! One-shot curved lettering. Shape the complete line before rotating clusters;
//! the result is raster content, with no persistent dependency on its guide.
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result, ensure};
use cosmic_text::{CacheKeyFlags, Command};
use image::RgbaImage;
use resvg::tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

use super::{TextAlign, TextLayout, TextSpec};
use crate::document::{MAX_IMAGE_BYTES, MAX_NEW_EDGE};
use crate::paths::VectorPath;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub align: TextAlign,
    pub offset: f64,
    // Positive is below the baseline in the direction of travel.
    pub baseline: f64,
    pub reverse: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            align: TextAlign::Left,
            offset: 0.,
            baseline: 0.,
            reverse: false,
        }
    }
}

pub struct Prepared {
    paths: Vec<Path>,
    color: image::Rgba<u8>,
    pub offset: (i32, i32),
    pub size: (u32, u32),
    pub overflow: bool,
}

struct Guide {
    points: Vec<(f64, f64)>,
    distances: Vec<f64>,
    length: f64,
}

impl Guide {
    fn new(path: &VectorPath, reverse: bool) -> Result<Self> {
        ensure!(
            path.anchors.len() >= 2
                && path.anchors.len() <= crate::paths::MAX_ANCHORS
                && path.anchors.iter().all(|a| a.finite()),
            "Choose a usable guide path."
        );
        let mut flattened = crate::paths::stroke(path);
        if reverse {
            flattened.reverse();
        }
        let mut points = Vec::with_capacity(flattened.len());
        let mut distances = vec![0.];
        let mut length = 0.;
        for point in flattened {
            let Some(previous) = points.last().copied() else {
                points.push(point);
                continue;
            };
            let next = length + (point.0 - previous.0).hypot(point.1 - previous.1);
            if next - length <= 1e-6 {
                continue;
            }
            length = next;
            ensure!(
                length.is_finite() && length <= 1e9,
                "The guide path is too large."
            );
            distances.push(length);
            points.push(point);
        }
        ensure!(length > 1e-6, "The guide path has no length.");
        Ok(Self {
            points,
            distances,
            length,
        })
    }

    fn sample(&self, distance: f64) -> ((f64, f64), (f64, f64)) {
        let distance = distance.clamp(0., self.length);
        let index = self
            .distances
            .partition_point(|value| *value <= distance)
            .clamp(1, self.points.len() - 1);
        let (a, b) = (self.points[index - 1], self.points[index]);
        let length = self.distances[index] - self.distances[index - 1];
        let tangent = ((b.0 - a.0) / length, (b.1 - a.1) / length);
        let along = distance - self.distances[index - 1];
        ((a.0 + tangent.0 * along, a.1 + tangent.1 * along), tangent)
    }
}

pub fn prepare(
    spec: &TextSpec,
    path: &VectorPath,
    options: Options,
    canceled: &AtomicBool,
) -> Result<Prepared> {
    ensure!(
        super::fields_allowed(
            &spec.source,
            &spec.family,
            &spec.family,
            spec.size,
            spec.spacing
        ) && spec.layout.allowed(),
        "The text settings are out of range."
    );
    ensure!(
        !spec.source.contains(['\n', '\r']),
        "Text Along Path supports one line. Remove line breaks and try again."
    );
    ensure!(
        options.offset.is_finite()
            && options.baseline.is_finite()
            && options.offset.abs() <= 1_000_000.
            && options.baseline.abs() <= 1_000_000.,
        "Offsets must be between −1000000 and 1000000 pixels."
    );
    ensure!(spec.color[3] != 0, "The text is fully transparent.");
    let guide = Guide::new(path, options.reverse)?;
    super::with_fonts(|fonts, cache| {
        // Fixed boxes and their clipping do not constrain a guide's single line.
        let buffer = super::layout(
            fonts,
            &spec.source,
            &spec.family,
            spec.size,
            spec.spacing,
            spec.face,
            TextLayout::default(),
        );
        let run = buffer
            .layout_runs()
            .next()
            .context("There is no text to place.")?;
        let width = f64::from(run.line_w);
        ensure!(
            width.is_finite() && width > 0.,
            "There is no text to place."
        );
        let start = options.offset
            + (guide.length - width)
                * match options.align {
                    TextAlign::Left => 0.,
                    TextAlign::Center => 0.5,
                    TextAlign::Right => 1.,
                };
        // A cluster can contain a base and several marks, or a ligature. All its
        // glyphs share one rotation and retain the shaper's relative positions.
        let mut clusters = BTreeMap::<(usize, usize), (f64, f64)>::new();
        for glyph in run.glyphs {
            let left = f64::from(glyph.x);
            let right = left + f64::from(glyph.w);
            let entry = clusters
                .entry((glyph.start, glyph.end))
                .or_insert((left, right));
            entry.0 = entry.0.min(left);
            entry.1 = entry.1.max(right);
        }
        let mut paths = Vec::new();
        // Keep outline storage local to this request. Font/size changes must not
        // grow the process-wide raster cache's optional outline cache forever.
        let mut outlines = HashMap::new();
        let mut overflow = start < -1e-6 || start + width > guide.length + 1e-6;
        let mut commands_count = 0usize;
        for glyph in run.glyphs {
            ensure!(
                !canceled.load(Ordering::Relaxed),
                "Text placement canceled."
            );
            let (left, right) = clusters[&(glyph.start, glyph.end)];
            // Omit complete overflowing clusters; never wrap a closed guide or
            // slice a combining sequence through the end of an open guide.
            if start + left < -1e-6 || start + right > guide.length + 1e-6 {
                overflow = true;
                continue;
            }
            let center = (left + right) * 0.5;
            let (position, tangent) = guide.sample(start + center);
            let mut key = glyph.physical((0., 0.), 1.).cache_key;
            key.flags |= CacheKeyFlags::DISABLE_HINTING;
            let commands = outlines
                .entry(key)
                .or_insert_with(|| cache.get_outline_commands_uncached(fonts, key));
            let Some(commands) = commands.as_deref() else {
                let source = &spec.source[glyph.start..glyph.end];
                ensure!(
                    source.chars().all(char::is_whitespace),
                    "A glyph has no scalable outline. Bitmap-only fonts and emoji are not supported by Text Along Path."
                );
                continue;
            };
            commands_count += commands.len();
            ensure!(
                commands_count <= 1_000_000,
                "The text outlines are too complex."
            );
            let map = |px: f32, py: f32| {
                let x =
                    f64::from(glyph.x + glyph.font_size * glyph.x_offset) + f64::from(px) - center;
                let y = f64::from(glyph.y - glyph.font_size * glyph.y_offset) - f64::from(py)
                    + options.baseline;
                (
                    (position.0 + tangent.0 * x - tangent.1 * y) as f32,
                    (position.1 + tangent.1 * x + tangent.0 * y) as f32,
                )
            };
            let mut builder = PathBuilder::new();
            for command in commands {
                match *command {
                    Command::MoveTo(p) => {
                        let (x, y) = map(p.x, p.y);
                        builder.move_to(x, y);
                    }
                    Command::LineTo(p) => {
                        let (x, y) = map(p.x, p.y);
                        builder.line_to(x, y);
                    }
                    Command::QuadTo(c, p) => {
                        let (cx, cy) = map(c.x, c.y);
                        let (x, y) = map(p.x, p.y);
                        builder.quad_to(cx, cy, x, y);
                    }
                    Command::CurveTo(a, b, p) => {
                        let (ax, ay) = map(a.x, a.y);
                        let (bx, by) = map(b.x, b.y);
                        let (x, y) = map(p.x, p.y);
                        builder.cubic_to(ax, ay, bx, by, x, y);
                    }
                    Command::Close => builder.close(),
                }
            }
            if let Some(path) = builder.finish() {
                paths.push(path);
            }
        }
        ensure!(
            !paths.is_empty(),
            "No letters fit on the guide. Reduce the font size, adjust the offset, or use a longer path."
        );
        let bounds = paths.iter().map(Path::bounds).fold(
            [
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ],
            |b, r| {
                [
                    b[0].min(f64::from(r.left())),
                    b[1].min(f64::from(r.top())),
                    b[2].max(f64::from(r.right())),
                    b[3].max(f64::from(r.bottom())),
                ]
            },
        );
        let left = bounds[0].floor() - 1.;
        let top = bounds[1].floor() - 1.;
        let width = bounds[2].ceil() + 1. - left;
        let height = bounds[3].ceil() + 1. - top;
        ensure!(
            left >= f64::from(i32::MIN)
                && top >= f64::from(i32::MIN)
                && left + width <= f64::from(i32::MAX)
                && top + height <= f64::from(i32::MAX)
                && width >= 1.
                && height >= 1.
                && width <= f64::from(MAX_NEW_EDGE)
                && height <= f64::from(MAX_NEW_EDGE)
                && width * height <= (MAX_IMAGE_BYTES / 4) as f64,
            "The curved text exceeds the layer size limit."
        );
        Ok(Prepared {
            paths,
            color: spec.color,
            offset: (left as i32, top as i32),
            size: (width as u32, height as u32),
            overflow,
        })
    })
}

impl Prepared {
    fn raster(&self, size: (u32, u32), transform: Transform) -> Result<RgbaImage> {
        let mut pixmap = Pixmap::new(size.0, size.1).context("The text bitmap is too large.")?;
        let mut paint = Paint::default();
        paint.set_color_rgba8(self.color[0], self.color[1], self.color[2], self.color[3]);
        for path in &self.paths {
            pixmap.fill_path(path, &paint, FillRule::Winding, transform, None);
        }
        let mut data = pixmap.take();
        for pixel in data.as_chunks_mut::<4>().0 {
            let alpha = u32::from(pixel[3]);
            for channel in &mut pixel[..3] {
                *channel = (u32::from(*channel) * 255 + alpha / 2)
                    .checked_div(alpha)
                    .unwrap_or(0)
                    .min(255) as u8;
            }
        }
        RgbaImage::from_raw(size.0, size.1, data).context("Invalid text bitmap.")
    }

    pub fn render(&self) -> Result<RgbaImage> {
        self.raster(
            self.size,
            Transform::from_translate(-(self.offset.0 as f32), -(self.offset.1 as f32)),
        )
    }

    pub fn preview(&self, canvas: (u32, u32), edge: u32) -> Result<RgbaImage> {
        let size = crate::adjustments::preview_size(canvas.0, canvas.1, edge.clamp(1, 1024));
        self.raster(
            size,
            Transform::from_scale(
                size.0 as f32 / canvas.0 as f32,
                size.1 as f32 / canvas.1 as f32,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::{Anchor, AnchorId, PathId};

    fn guide() -> VectorPath {
        VectorPath {
            id: PathId(1),
            visible: true,
            closed: false,
            anchors: vec![
                Anchor::corner(AnchorId(1), (20., 80.)),
                Anchor::corner(AnchorId(2), (420., 80.)),
            ],
        }
    }

    fn spec(source: &str) -> TextSpec {
        TextSpec {
            source: source.into(),
            family: "sans-serif".into(),
            face: Default::default(),
            size: 32.,
            color: image::Rgba([30, 80, 150, 180]),
            spacing: Default::default(),
            layout: Default::default(),
        }
    }

    #[test]
    fn guide_uses_distance_instead_of_bezier_parameter_and_reverses_without_changing_geometry() {
        let mut path = guide();
        path.anchors
            .insert(1, Anchor::corner(AnchorId(3), (20., 80.)));
        let normal = Guide::new(&path, false).unwrap();
        let reversed = Guide::new(&path, true).unwrap();
        assert_eq!(normal.length, 400.);
        assert_eq!(normal.sample(100.), ((120., 80.), (1., 0.)));
        assert_eq!(reversed.sample(300.), ((120., 80.), (-1., 0.)));
        path.anchors[0].handle_out = (20., -80.);
        path.anchors[2].handle_in = (420., -80.);
        let curved = Guide::new(&path, false).unwrap();
        assert!(curved.length > 400.);
        let (_, tangent) = curved.sample(curved.length * 0.5);
        assert!((tangent.0.hypot(tangent.1) - 1.).abs() < 1e-9);
        let zero = VectorPath {
            anchors: vec![path.anchors[1]; 2],
            ..guide()
        };
        assert!(Guide::new(&zero, false).is_err());
    }

    #[test]
    fn alignment_offsets_reverse_and_native_preview_preserve_curved_letter_coverage() {
        let canceled = AtomicBool::new(false);
        // Combining marks and a possible ligature exercise shaped glyph clusters.
        let text = spec("A\u{0301} fi OB");
        let normal = prepare(&text, &guide(), Options::default(), &canceled).unwrap();
        let shifted = prepare(
            &text,
            &guide(),
            Options {
                offset: 12.,
                baseline: 7.,
                ..Options::default()
            },
            &canceled,
        )
        .unwrap();
        assert_eq!(shifted.offset, (normal.offset.0 + 12, normal.offset.1 + 7));
        assert!(normal.render().unwrap() == shifted.render().unwrap());
        let end = prepare(
            &text,
            &guide(),
            Options {
                align: TextAlign::Right,
                ..Options::default()
            },
            &canceled,
        )
        .unwrap();
        assert!(end.offset.0 > normal.offset.0);
        let reverse = prepare(
            &text,
            &guide(),
            Options {
                reverse: true,
                ..Options::default()
            },
            &canceled,
        )
        .unwrap();
        assert!(reverse.offset.0 > normal.offset.0);
        assert!(reverse.offset.1 >= 78);
        let image = normal.render().unwrap();
        assert!(image.pixels().any(|pixel| pixel[3] > 0 && pixel[3] < 180));
        assert!(image.pixels().all(|pixel| pixel[3] <= 180));
        let mut placed = RgbaImage::new(512, 128);
        for (x, y, pixel) in image.enumerate_pixels() {
            placed.put_pixel(
                x + normal.offset.0 as u32,
                y + normal.offset.1 as u32,
                *pixel,
            );
        }
        let preview = normal.preview((512, 128), 512).unwrap();
        assert_eq!(
            preview
                .pixels()
                .zip(placed.pixels())
                .enumerate()
                .find(|(_, (a, b))| a != b),
            None
        );
        assert_eq!(
            normal.preview((16_384, 16_384), 128).unwrap().dimensions(),
            (128, 128)
        );
    }

    #[test]
    fn overflow_is_explicit_closed_paths_do_not_repeat_and_invalid_requests_allocate_no_bitmap() {
        let canceled = AtomicBool::new(false);
        let mut path = guide();
        path.anchors[1].point.0 = 90.;
        path.anchors[1].handle_in = path.anchors[1].point;
        path.anchors[1].handle_out = path.anchors[1].point;
        let result = prepare(
            &spec("Several letters"),
            &path,
            Options::default(),
            &canceled,
        )
        .unwrap();
        assert!(result.overflow);
        assert!(result.size.0 < 100);
        assert!(
            prepare(
                &spec("A"),
                &path,
                Options {
                    offset: 100.,
                    ..Options::default()
                },
                &canceled
            )
            .is_err()
        );
        assert!(prepare(&spec("two\nlines"), &path, Options::default(), &canceled).is_err());
        assert!(
            prepare(
                &spec("A"),
                &path,
                Options {
                    baseline: f64::NAN,
                    ..Options::default()
                },
                &canceled
            )
            .is_err()
        );
        path.closed = true;
        let result = prepare(
            &spec(&"O".repeat(100)),
            &path,
            Options::default(),
            &canceled,
        )
        .unwrap();
        assert!(result.overflow);
        canceled.store(true, Ordering::Relaxed);
        assert!(prepare(&spec("A"), &path, Options::default(), &canceled).is_err());
    }
}
