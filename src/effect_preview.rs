use std::sync::OnceLock;

use image::{Rgba, RgbaImage};

use crate::adjustments::{self, Adjustment};
use crate::document::{self, DocumentContent, LayerId};
use crate::filters::{self, FilterSpec};
use crate::selection::Selection;

const MAX_DISPLAY_EDGE: u32 = 1024;

// Fit the image to the actual logical preview box, then account for Retina/DPI.
// Never enlarge the source or let a large screen create unbounded background work.
pub fn display_edge(canvas: (u32, u32), bounds: (f32, f32), scale: f32) -> u32 {
    let fit = (bounds.0 / canvas.0.max(1) as f32).min(bounds.1 / canvas.1.max(1) as f32);
    let edge = canvas.0.max(canvas.1) as f32 * fit * scale;
    if edge.is_finite() {
        (edge.ceil() as u32).clamp(1, MAX_DISPLAY_EDGE)
    } else {
        MAX_DISPLAY_EDGE
    }
}

// An immutable dialog snapshot. Area reduction visits visible source pixels in
// background work; scratch space is bounded to source/preview rows, never a full canvas.
pub struct EffectPreview {
    content: DocumentContent,
    layer: LayerId,
    selection: Selection,
    size: (u32, u32),
    sampling_size: (u32, u32),
    original: OnceLock<RgbaImage>,
}

struct Region {
    pixels: RgbaImage,
    origin: (i64, i64),
    scale: f64,
}

struct LayerFilterPreview {
    image: filters::LayerFilterImage,
    origin: (i64, i64),
    scale: f64,
}

impl EffectPreview {
    pub fn new(content: DocumentContent, layer: LayerId, selection: Selection, edge: u32) -> Self {
        let (w, h) = content.size();
        let edge = edge.clamp(1, MAX_DISPLAY_EDGE);
        Self {
            content,
            layer,
            selection,
            size: adjustments::preview_size(w, h, edge),
            sampling_size: adjustments::preview_size(w, h, edge * 2),
            original: OnceLock::new(),
        }
    }

    pub fn resized(&self, edge: u32) -> Option<Self> {
        let edge = edge.clamp(1, MAX_DISPLAY_EDGE);
        let (w, h) = self.content.size();
        (adjustments::preview_size(w, h, edge) != self.size).then(|| {
            Self::new(
                self.content.clone(),
                self.layer,
                self.selection.clone(),
                edge,
            )
        })
    }

    pub fn original(&self) -> &RgbaImage {
        self.original
            .get_or_init(|| self.composite(|pixel, _, _| pixel, None))
    }

    pub fn adjustment(&self, adjustment: Adjustment) -> RgbaImage {
        if adjustment.is_identity() {
            return self.original().clone();
        }
        let prepared = adjustments::Prepared::from(adjustment);
        let alpha_locked = self
            .content
            .layer(self.layer)
            .is_some_and(|layer| layer.locks.alpha);
        self.composite(
            |pixel, x, y| {
                if alpha_locked && pixel[3] == 0 {
                    return pixel;
                }
                adjustments::blend_coverage(
                    pixel,
                    self.content.mode().constrain(prepared.map(pixel)),
                    self.coverage(x, y),
                )
            },
            None,
        )
    }

    pub fn filter(&self, spec: FilterSpec) -> RgbaImage {
        if spec.is_identity() {
            return self.original().clone();
        }
        if let FilterSpec::Shadow {
            radius,
            offset,
            opacity,
            color,
        } = spec
        {
            let Some(region) = self.region(filters::kernel_support(radius), offset, true) else {
                return self.original().clone();
            };
            let shadow = filters::render_shadow(
                &region.pixels,
                (f64::from(radius) * region.scale).round() as u32,
                opacity,
                self.content.mode().constrain(color),
            )
            .map(|image| LayerFilterPreview {
                image,
                origin: region.origin,
                scale: region.scale,
            });
            return self.composite(|pixel, _, _| pixel, shadow.as_ref());
        }
        if let FilterSpec::Stroke {
            width,
            opacity,
            color,
        } = spec
        {
            let Some(region) = self.region(width, (0, 0), true) else {
                return self.original().clone();
            };
            let stroke = filters::render_stroke_scaled(
                &region.pixels,
                f64::from(width) * region.scale,
                opacity,
                self.content.mode().constrain(color),
            )
            .map(|image| LayerFilterPreview {
                image,
                origin: region.origin,
                scale: region.scale,
            });
            return self.composite(|pixel, _, _| pixel, stroke.as_ref());
        }
        let radius = match spec {
            FilterSpec::Blur { radius } | FilterSpec::Sharpen { radius, .. } => radius,
            _ => 0,
        };
        let Some(region) = self.region(filters::kernel_support(radius), (0, 0), false) else {
            return self.original().clone();
        };
        let radius = (f64::from(radius) * region.scale).round() as u32;
        // A subpixel radius cannot be represented in this reduced preview.
        // Leave it unchanged instead of showing a resampling artifact as an effect.
        if radius == 0 {
            return self.original().clone();
        }
        let alpha_locked = self
            .content
            .layer(self.layer)
            .is_some_and(|layer| layer.locks.alpha);
        let filtered = filters::preview(
            &region.pixels,
            match spec {
                FilterSpec::Blur { .. } => FilterSpec::Blur { radius },
                FilterSpec::Sharpen { amount, .. } => FilterSpec::Sharpen { radius, amount },
                _ => unreachable!(),
            },
            0,
            |_, _| 255,
            |pixel| self.content.mode().constrain(pixel),
            alpha_locked,
        );
        self.composite(
            |pixel, x, y| {
                if alpha_locked && pixel[3] == 0 {
                    return pixel;
                }
                let sx = ((x - region.origin.0) as f64 * region.scale).floor() as u32;
                let sy = ((y - region.origin.1) as f64 * region.scale).floor() as u32;
                let target = *filtered
                    .get_pixel(sx.min(filtered.width() - 1), sy.min(filtered.height() - 1));
                let target = if alpha_locked {
                    crate::paint::preserve_alpha(pixel, target)
                } else {
                    target
                };
                filters::blend(pixel, target, self.coverage(x, y))
            },
            None,
        )
    }

    // Retain the visible source plus the filter's complete sampling halo. This
    // avoids squeezing a huge layer into a tiny preview of a small canvas, and
    // includes off-canvas pixels that can blur or cast a shadow onto the canvas.
    fn region(&self, halo: u32, offset: (i32, i32), masked: bool) -> Option<Region> {
        let layer = self.content.layer(self.layer)?;
        let canvas = self.content.size();
        let scale = f64::from(self.sampling_size.0.max(self.sampling_size.1))
            / f64::from(canvas.0.max(canvas.1));
        let pad = i64::from(halo);
        let origin = (
            i64::from(layer.offset.0) + i64::from(offset.0),
            i64::from(layer.offset.1) + i64::from(offset.1),
        );
        let left = (-origin.0 - pad).clamp(0, i64::from(layer.pixels.width()));
        let top = (-origin.1 - pad).clamp(0, i64::from(layer.pixels.height()));
        let right =
            (i64::from(canvas.0) - origin.0 + pad).clamp(0, i64::from(layer.pixels.width()));
        let bottom =
            (i64::from(canvas.1) - origin.1 + pad).clamp(0, i64::from(layer.pixels.height()));
        if right <= left || bottom <= top {
            return None;
        }
        let w = ((right - left) as f64 * scale).ceil().max(1.) as u32;
        let h = ((bottom - top) as f64 * scale).ceil().max(1.) as u32;
        let pixels = reduce_area(
            ((right - left) as u32, (bottom - top) as u32),
            (w, h),
            |y, row| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let (sx, sy) = (left as u32 + x as u32, top as u32 + y);
                    *pixel = *layer.pixels.get_pixel(sx, sy);
                    if masked {
                        *pixel = layer.masked(*pixel, sx, sy);
                    }
                }
            },
        );
        Some(Region {
            pixels,
            origin: (origin.0 + left, origin.1 + top),
            scale,
        })
    }

    fn coverage(&self, x: i64, y: i64) -> u8 {
        let size = self.content.size();
        self.selection.coverage(x, y, size.0, size.1)
    }

    fn composite(
        &self,
        map: impl Fn(Rgba<u8>, i64, i64) -> Rgba<u8>,
        shadow: Option<&LayerFilterPreview>,
    ) -> RgbaImage {
        self.composite_with_replacement(map, shadow, None)
    }

    // A bounded, canvas-sized raster replaces the text source at its stack slot.
    // At native size this is the same composition as inserting above a hidden
    // source. Reduced previews sample the rendered curves at preview resolution.
    pub fn replacement(&self, image: &RgbaImage) -> RgbaImage {
        self.composite_with_replacement(|pixel, _, _| pixel, None, Some(image))
    }

    fn composite_with_replacement(
        &self,
        map: impl Fn(Rgba<u8>, i64, i64) -> Rgba<u8>,
        shadow: Option<&LayerFilterPreview>,
        replacement: Option<&RgbaImage>,
    ) -> RgbaImage {
        let canvas = self.content.size();
        reduce_area(canvas, self.size, |cy, row| {
            row.fill(Rgba([0; 4]));
            for layer in self.content.layers() {
                if layer.id == self.layer
                    && let Some(image) = replacement
                {
                    let y =
                        (u64::from(cy) * u64::from(image.height()) / u64::from(canvas.1)) as u32;
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let x = (x as u64 * u64::from(image.width()) / u64::from(canvas.0)) as u32;
                        *pixel = document::blend(
                            *pixel,
                            self.content.mode().constrain(*image.get_pixel(x, y)),
                            layer.opacity,
                            layer.blend_mode,
                        );
                    }
                    continue;
                }
                // Generated silhouette layers are independent of source opacity and selection.
                if layer.id == self.layer
                    && let Some(shadow) = shadow
                {
                    let sy = ((i64::from(cy) - shadow.origin.1) as f64 * shadow.scale).floor()
                        as i64
                        + i64::from(shadow.image.pad);
                    if (0..i64::from(shadow.image.pixels.height())).contains(&sy) {
                        for (cx, pixel) in row.iter_mut().enumerate() {
                            let sx = ((cx as i64 - shadow.origin.0) as f64 * shadow.scale).floor()
                                as i64
                                + i64::from(shadow.image.pad);
                            if (0..i64::from(shadow.image.pixels.width())).contains(&sx) {
                                *pixel = document::source_over(
                                    *pixel,
                                    *shadow.image.pixels.get_pixel(sx as u32, sy as u32),
                                    255,
                                );
                            }
                        }
                    }
                }
                if !layer.visible || layer.opacity == 0 {
                    continue;
                }
                let sy = i64::from(cy) - i64::from(layer.offset.1);
                if !(0..i64::from(layer.pixels.height())).contains(&sy) {
                    continue;
                }
                let left = i64::from(layer.offset.0).clamp(0, i64::from(canvas.0)) as usize;
                let right = (i64::from(layer.offset.0) + i64::from(layer.pixels.width()))
                    .clamp(0, i64::from(canvas.0)) as usize;
                for (cx, destination) in row.iter_mut().enumerate().take(right).skip(left) {
                    let sx = (cx as i64 - i64::from(layer.offset.0)) as u32;
                    let mut pixel = *layer.pixels.get_pixel(sx, sy as u32);
                    if layer.id == self.layer {
                        pixel = map(pixel, cx as i64, i64::from(cy));
                    }
                    pixel = layer.masked(pixel, sx, sy as u32);
                    *destination =
                        document::blend(*destination, pixel, layer.opacity, layer.blend_mode);
                }
            }
        })
    }
}

// Exact box-area reduction with premultiplied color. Every contributing source
// pixel is read before decimation, so periodic fine detail cannot alias away.
// Cache one horizontal row across adjacent output rows; memory is O(source width
// + output width), besides the output bitmap. Native-size paths retain exact bytes.
fn reduce_area(
    source: (u32, u32),
    target: (u32, u32),
    mut render_row: impl FnMut(u32, &mut [Rgba<u8>]),
) -> RgbaImage {
    let mut output = RgbaImage::new(target.0, target.1);
    let mut row = vec![Rgba([0; 4]); source.0 as usize];
    if source == target {
        for y in 0..source.1 {
            render_row(y, &mut row);
            for (x, pixel) in row.iter().enumerate() {
                output.put_pixel(x as u32, y, *pixel);
            }
        }
        return output;
    }
    let mut horizontal = vec![[0_f64; 4]; target.0 as usize];
    let mut vertical = horizontal.clone();
    let mut cached_y = None;
    let dx = f64::from(source.0) / f64::from(target.0);
    let dy = f64::from(source.1) / f64::from(target.1);
    for oy in 0..target.1 {
        vertical.fill([0.; 4]);
        let (top, bottom) = (f64::from(oy) * dy, f64::from(oy + 1) * dy);
        for sy in top.floor() as u32..(bottom.ceil() as u32).min(source.1) {
            if cached_y != Some(sy) {
                render_row(sy, &mut row);
                for ox in 0..target.0 {
                    let (left, right) = (f64::from(ox) * dx, f64::from(ox + 1) * dx);
                    let mut sum = [0.; 4];
                    for sx in left.floor() as u32..(right.ceil() as u32).min(source.0) {
                        let weight =
                            (right.min(f64::from(sx + 1)) - left.max(f64::from(sx))).max(0.);
                        let p = row[sx as usize];
                        let alpha = f64::from(p[3]) * weight;
                        for c in 0..3 {
                            sum[c] += f64::from(p[c]) * alpha;
                        }
                        sum[3] += alpha;
                    }
                    horizontal[ox as usize] = sum.map(|v| v / dx);
                }
                cached_y = Some(sy);
            }
            let weight = (bottom.min(f64::from(sy + 1)) - top.max(f64::from(sy))).max(0.);
            for (sum, source) in vertical.iter_mut().zip(&horizontal) {
                for c in 0..4 {
                    sum[c] += source[c] * weight;
                }
            }
        }
        for (ox, sum) in vertical.iter().enumerate() {
            let alpha = (sum[3] / dy).round().clamp(0., 255.) as u8;
            let pixel = if alpha == 0 {
                Rgba([0; 4])
            } else {
                Rgba([
                    (sum[0] / sum[3]).round() as u8,
                    (sum[1] / sum[3]).round() as u8,
                    (sum[2] / sum[3]).round() as u8,
                    alpha,
                ])
            };
            output.put_pixel(ox as u32, oy, pixel);
        }
    }
    output
}

pub fn checker(image: &RgbaImage) -> RgbaImage {
    RgbaImage::from_fn(image.width(), image.height(), |x, y| {
        let gray = if (x / 6 + y / 6) % 2 == 0 { 66 } else { 88 };
        document::source_over(Rgba([gray, gray, gray, 255]), *image.get_pixel(x, y), 255)
    })
}
