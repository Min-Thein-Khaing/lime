//! Outside-only grayscale alpha dilation with a round, subpixel-sampled kernel.
//! Scratch storage is one row; the output alpha doubles as the dilation buffer.
use std::collections::VecDeque;

use image::{Rgba, RgbaImage};

use super::LayerFilterImage;

pub const MAX_STROKE_WIDTH: u32 = 128;

pub fn stroke_dimensions(width: u32, height: u32, size: u32) -> Option<(u32, u32)> {
    if size > MAX_STROKE_WIDTH {
        return None;
    }
    Some((
        width.checked_add(size.checked_mul(2)?)?,
        height.checked_add(size.checked_mul(2)?)?,
    ))
}

// Minkowski expansion of a source pixel's square by a disk. Four-by-four
// integration keeps the outer corners smooth without thresholding source alpha.
fn kernel_coverage(x: i32, y: i32, radius: f64) -> u8 {
    // Integrate straight axial edges exactly, including very thin reduced previews.
    if x == 0 || y == 0 {
        return ((radius - f64::from(x.abs().max(y.abs())) + 1.0).clamp(0.0, 1.0) * 255.0).round()
            as u8;
    }
    let mut hits = 0u32;
    for sy in 0..4 {
        for sx in 0..4 {
            let dx = ((x as f64 + (sx as f64 + 0.5) / 4.0 - 0.5).abs() - 0.5).max(0.0);
            let dy = ((y as f64 + (sy as f64 + 0.5) / 4.0 - 0.5).abs() - 0.5).max(0.0);
            hits += u32::from(dx * dx + dy * dy <= radius * radius);
        }
    }
    ((hits * 255 + 8) / 16) as u8
}

pub fn render_stroke(
    source: &RgbaImage,
    width: u32,
    opacity: u8,
    color: Rgba<u8>,
) -> Option<LayerFilterImage> {
    render_stroke_scaled(source, f64::from(width), opacity, color)
}

pub(crate) fn render_stroke_scaled(
    source: &RgbaImage,
    size: f64,
    opacity: u8,
    color: Rgba<u8>,
) -> Option<LayerFilterImage> {
    if !size.is_finite() || size <= 0.0 || size > f64::from(MAX_STROKE_WIDTH) || opacity == 0 {
        return None;
    }
    let max_alpha = source.pixels().map(|pixel| pixel[3]).max().unwrap_or(0);
    if max_alpha == 0 {
        return None;
    }
    let width = size.ceil() as u32;
    let (w, h) = stroke_dimensions(source.width(), source.height(), width)?;
    let mut output = RgbaImage::new(w, h);
    let radius = width as i32;
    let mut queue = VecDeque::<u32>::with_capacity(source.width() as usize);
    for dy in -radius..=radius {
        let full = (f64::from(dy.abs()) <= size)
            .then(|| (size * size - f64::from(dy * dy)).sqrt().floor() as u32);
        let fringe: Vec<_> = (full.map_or(0, |value| value + 1)..=width)
            .filter_map(|dx| {
                let coverage = kernel_coverage(dx as i32, dy, size);
                (coverage != 0).then_some((dx, coverage))
            })
            .collect();
        for sy in 0..source.height() {
            let y = (sy as i64 + i64::from(width) + i64::from(dy)) as u32;
            queue.clear();
            let mut next = 0;
            for x in 0..w {
                let center = i64::from(x) - i64::from(width);
                let left = (center - i64::from(full.unwrap_or(0))).max(0) as u32;
                let right = full.map_or(0, |full| {
                    (center + i64::from(full) + 1).clamp(0, i64::from(source.width())) as u32
                });
                while next < right {
                    let alpha = source.get_pixel(next, sy)[3];
                    while queue
                        .back()
                        .is_some_and(|i| source.get_pixel(*i, sy)[3] <= alpha)
                    {
                        queue.pop_back();
                    }
                    queue.push_back(next);
                    next += 1;
                }
                while queue.front().is_some_and(|i| *i < left) {
                    queue.pop_front();
                }
                let mut alpha = queue.front().map_or(0, |i| source.get_pixel(*i, sy)[3]);
                let destination = output.get_pixel_mut(x, y);
                if destination[3] == max_alpha {
                    continue;
                }
                if alpha == max_alpha {
                    destination[3] = alpha;
                    continue;
                }
                for &(dx, coverage) in &fringe {
                    for sx in [center - i64::from(dx), center + i64::from(dx)] {
                        if (0..i64::from(source.width())).contains(&sx) {
                            let value = (u32::from(source.get_pixel(sx as u32, sy)[3])
                                * u32::from(coverage)
                                + 127)
                                / 255;
                            alpha = alpha.max(value as u8);
                        }
                    }
                }
                destination[3] = destination[3].max(alpha);
            }
        }
    }
    let mut visible = false;
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let original = x
            .checked_sub(width)
            .zip(y.checked_sub(width))
            .filter(|(sx, sy)| *sx < source.width() && *sy < source.height())
            .map_or(0, |(sx, sy)| source.get_pixel(sx, sy)[3]);
        let alpha =
            (u32::from(pixel[3].saturating_sub(original)) * u32::from(opacity.min(100)) + 50) / 100;
        if alpha != 0 {
            *pixel = Rgba([color[0], color[1], color[2], alpha as u8]);
            visible = true;
        } else {
            *pixel = Rgba([0; 4]);
        }
    }
    visible.then_some(LayerFilterImage {
        pixels: output,
        pad: radius,
    })
}
