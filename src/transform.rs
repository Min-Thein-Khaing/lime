use image::{ImageBuffer, Rgba, RgbaImage, imageops::FilterType};

use crate::document::{MAX_IMAGE_BYTES, MAX_NEW_EDGE};
use crate::preview::PixelRect;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    // 90 degrees counter-clockwise.
    Left,
    // 90 degrees clockwise.
    Right,
    Half,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mirror {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Resample {
    Nearest,
    Bilinear,
    Bicubic,
}

// Where the existing canvas stays put when the canvas grows or shrinks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Anchor {
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Anchor {
    pub const ALL: [Self; 9] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::West,
        Self::Center,
        Self::East,
        Self::SouthWest,
        Self::South,
        Self::SouthEast,
    ];

    pub fn from_grid(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    pub fn grid_index(self) -> usize {
        Self::ALL
            .iter()
            .position(|anchor| *anchor == self)
            .expect("all anchors are represented")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    Rgb,
    Grayscale,
}

impl ColorMode {
    pub fn constrain(self, color: Rgba<u8>) -> Rgba<u8> {
        match self {
            Self::Rgb => color,
            Self::Grayscale => gray_pixel(color),
        }
    }

    pub fn constrain_image(self, image: RgbaImage) -> RgbaImage {
        match self {
            Self::Rgb => image,
            Self::Grayscale => grayscale(image),
        }
    }
}

pub fn canvas_allowed(width: u32, height: u32) -> bool {
    (1..=MAX_NEW_EDGE).contains(&width)
        && (1..=MAX_NEW_EDGE).contains(&height)
        && u64::from(width)
            .saturating_mul(u64::from(height))
            .saturating_mul(4)
            <= MAX_IMAGE_BYTES
}

pub fn turned_size(width: u32, height: u32, turn: Turn) -> (u32, u32) {
    match turn {
        Turn::Half => (width, height),
        Turn::Left | Turn::Right => (height, width),
    }
}

pub fn turn_pixels(pixels: &RgbaImage, turn: Turn) -> RgbaImage {
    match turn {
        Turn::Right => image::imageops::rotate90(pixels),
        Turn::Left => image::imageops::rotate270(pixels),
        Turn::Half => image::imageops::rotate180(pixels),
    }
}

pub fn mirror_pixels(pixels: &RgbaImage, mirror: Mirror) -> RgbaImage {
    match mirror {
        Mirror::Horizontal => image::imageops::flip_horizontal(pixels),
        Mirror::Vertical => image::imageops::flip_vertical(pixels),
    }
}

fn fit_offset(x: i64, y: i64) -> Option<(i32, i32)> {
    Some((i32::try_from(x).ok()?, i32::try_from(y).ok()?))
}

pub fn turned_offset(
    offset: (i32, i32),
    layer: (u32, u32),
    canvas: (u32, u32),
    turn: Turn,
) -> Option<(i32, i32)> {
    let (ox, oy) = (i64::from(offset.0), i64::from(offset.1));
    let (lw, lh) = (i64::from(layer.0), i64::from(layer.1));
    let (width, height) = (i64::from(canvas.0), i64::from(canvas.1));
    let (x, y) = match turn {
        Turn::Right => (height - oy - lh, ox),
        Turn::Left => (oy, width - ox - lw),
        Turn::Half => (width - ox - lw, height - oy - lh),
    };
    fit_offset(x, y)
}

pub fn mirrored_offset(
    offset: (i32, i32),
    layer: (u32, u32),
    canvas: (u32, u32),
    mirror: Mirror,
) -> Option<(i32, i32)> {
    let (ox, oy) = (i64::from(offset.0), i64::from(offset.1));
    let (lw, lh) = (i64::from(layer.0), i64::from(layer.1));
    let (width, height) = (i64::from(canvas.0), i64::from(canvas.1));
    let (x, y) = match mirror {
        Mirror::Horizontal => (width - ox - lw, oy),
        Mirror::Vertical => (ox, height - oy - lh),
    };
    fit_offset(x, y)
}

// Keeps the layer's center, rounding half pixels away from zero.
pub fn centered_offset(offset: (i32, i32), old: (u32, u32), new: (u32, u32)) -> (i32, i32) {
    let place = |origin: i32, before: u32, after: u32| {
        let value = f64::from(origin) + f64::from(before) / 2.0 - f64::from(after) / 2.0;
        value
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
    };
    (place(offset.0, old.0, new.0), place(offset.1, old.1, new.1))
}

pub fn anchored_offset(
    offset: (i32, i32),
    old: (u32, u32),
    new: (u32, u32),
    anchor: Anchor,
) -> Option<(i32, i32)> {
    if old.0 == 0 || old.1 == 0 || new.0 == 0 || new.1 == 0 {
        return None;
    }
    let index = anchor.grid_index();
    let axis = |origin: i32, before: u32, after: u32, at: usize| {
        let value = f64::from(origin) + (f64::from(before) - f64::from(after)) * at as f64 / 2.;
        as_i32(value.round())
    };
    Some((
        axis(offset.0, old.0, new.0, index % 3)?,
        axis(offset.1, old.1, new.1, index / 3)?,
    ))
}

pub fn anchor_delta(old: (u32, u32), new: (u32, u32), anchor: Anchor) -> (i32, i32) {
    let place = |before: u32, after: u32, at: u8| match at {
        0 => 0,
        1 => i64::from(after) / 2 - i64::from(before) / 2,
        _ => i64::from(after) - i64::from(before),
    };
    let (x, y) = match anchor {
        Anchor::NorthWest => (0, 0),
        Anchor::North => (1, 0),
        Anchor::NorthEast => (2, 0),
        Anchor::West => (0, 1),
        Anchor::Center => (1, 1),
        Anchor::East => (2, 1),
        Anchor::SouthWest => (0, 2),
        Anchor::South => (1, 2),
        Anchor::SouthEast => (2, 2),
    };
    (place(old.0, new.0, x) as i32, place(old.1, new.1, y) as i32)
}

pub fn turn_rect(rect: PixelRect, canvas: (u32, u32), turn: Turn) -> PixelRect {
    let (x, y) = (rect.x, rect.y);
    let (w, h) = (i64::from(rect.width), i64::from(rect.height));
    let (width, height) = (i64::from(canvas.0), i64::from(canvas.1));
    let (nx, ny, nw, nh) = match turn {
        Turn::Right => (height - (y + h), x, h, w),
        Turn::Left => (y, width - (x + w), h, w),
        Turn::Half => (width - (x + w), height - (y + h), w, h),
    };
    PixelRect::new(nx, ny, nw as u32, nh as u32)
}

pub fn mirror_rect(rect: PixelRect, canvas: (u32, u32), mirror: Mirror) -> PixelRect {
    let (x, y) = (rect.x, rect.y);
    let (w, h) = (i64::from(rect.width), i64::from(rect.height));
    let (width, height) = (i64::from(canvas.0), i64::from(canvas.1));
    let (nx, ny) = match mirror {
        Mirror::Horizontal => (width - (x + w), y),
        Mirror::Vertical => (x, height - (y + h)),
    };
    PixelRect::new(nx, ny, rect.width, rect.height)
}

pub fn scale_rect(rect: PixelRect, old: (u32, u32), new: (u32, u32)) -> PixelRect {
    let (x, width) = scale_span(rect.x, rect.width, old.0, new.0);
    let (y, height) = scale_span(rect.y, rect.height, old.1, new.1);
    PixelRect::new(x, y, width, height)
}

// The layer's new origin and pixel size when the canvas scale is applied to its edges.
pub fn scale_placement(
    offset: (i32, i32),
    layer: (u32, u32),
    old: (u32, u32),
    new: (u32, u32),
) -> Option<((i32, i32), (u32, u32))> {
    let (x, width) = scale_span(i64::from(offset.0), layer.0, old.0, new.0);
    let (y, height) = scale_span(i64::from(offset.1), layer.1, old.1, new.1);
    Some((
        (i32::try_from(x).ok()?, i32::try_from(y).ok()?),
        (width.max(1), height.max(1)),
    ))
}

pub fn scale_pixels(pixels: &RgbaImage, width: u32, height: u32, resample: Resample) -> RgbaImage {
    if pixels.dimensions() == (width, height) {
        return pixels.clone();
    }
    match resample {
        Resample::Nearest => image::imageops::resize(pixels, width, height, FilterType::Nearest),
        Resample::Bilinear | Resample::Bicubic => {
            let filter = match resample {
                Resample::Bicubic => FilterType::CatmullRom,
                _ => FilterType::Triangle,
            };
            let scaled = image::imageops::resize(&premultiply(pixels), width, height, filter);
            unpremultiply(&scaled)
        }
    }
}

pub fn gray_pixel(color: Rgba<u8>) -> Rgba<u8> {
    let [r, g, b, a] = color.0;
    let gray = ((u16::from(r) * 77 + u16::from(g) * 150 + u16::from(b) * 29 + 128) >> 8) as u8;
    Rgba([gray, gray, gray, a])
}

pub fn grayscale(mut image: RgbaImage) -> RgbaImage {
    for pixel in image.pixels_mut() {
        *pixel = gray_pixel(*pixel);
    }
    image
}

fn scale_span(start: i64, length: u32, old: u32, new: u32) -> (i64, u32) {
    let end = start.saturating_add(i64::from(length));
    let map = |value: i64| value.saturating_mul(i64::from(new)) / i64::from(old);
    let mapped = map(start);
    let width = map(end).saturating_sub(mapped);
    (mapped, u32::try_from(width).unwrap_or(u32::MAX))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Warp {
    Rotate(f64),
    RotateAround { degrees: f64, pivot: (f64, f64) },
    Flip(Mirror),
    Perspective([(f64, f64); 4]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarpError {
    Singular,
    Crossing,
    Concave,
    TooLarge,
}

impl WarpError {
    pub fn message(self) -> &'static str {
        match self {
            Self::Singular => "That shape is too flat to transform.",
            Self::Crossing => "The corners cross. Move them so the edges do not intersect.",
            Self::Concave => "Keep all four corners on the outside of the shape.",
            Self::TooLarge => "The transformed layer is too large.",
        }
    }
}

pub struct Placed {
    pub pixels: RgbaImage,
    pub offset: (i32, i32),
    pub bounds: (u32, u32),
}

pub fn layer_corners(offset: (i32, i32), size: (u32, u32)) -> [(f64, f64); 4] {
    let x = f64::from(offset.0);
    let y = f64::from(offset.1);
    let w = f64::from(size.0);
    let h = f64::from(size.1);
    [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlacedBox {
    pub offset: (i32, i32),
    pub size: (u32, u32),
}

pub fn warp_bounds(
    offset: (i32, i32),
    size: (u32, u32),
    warp: Warp,
) -> Result<PlacedBox, WarpError> {
    if size.0 == 0 || size.1 == 0 {
        return Err(WarpError::Singular);
    }
    let corners = match warp {
        Warp::Rotate(degrees) => {
            if !degrees.is_finite() {
                return Err(WarpError::Singular);
            }
            if let Some(turn) = exact_turn(degrees) {
                let turned = turned_size(size.0, size.1, turn);
                return Ok(PlacedBox {
                    offset: centered_offset(offset, size, turned),
                    size: turned,
                });
            }
            rotate_corners(layer_corners(offset, size), degrees)
        }
        Warp::RotateAround { degrees, pivot } => {
            if !degrees.is_finite() || !pivot.0.is_finite() || !pivot.1.is_finite() {
                return Err(WarpError::Singular);
            }
            if degrees.rem_euclid(360.) == 0. {
                if !canvas_allowed(size.0, size.1) {
                    return Err(WarpError::TooLarge);
                }
                return Ok(PlacedBox { offset, size });
            }
            rotate_corners_about(layer_corners(offset, size), degrees, pivot)
        }
        Warp::Flip(_) => layer_corners(offset, size),
        Warp::Perspective(corners) => {
            validate_quad(corners)?;
            corners
        }
    };
    bounds_from_corners(corners)
}

// `max_edge` rasterizes a coarser preview into the same bounds. `None` writes every pixel.
pub fn warp_layer(
    pixels: &RgbaImage,
    offset: (i32, i32),
    warp: Warp,
    resample: Resample,
    max_edge: Option<u32>,
) -> Result<Placed, WarpError> {
    let size = pixels.dimensions();
    let PlacedBox {
        offset: placed_offset,
        size: bounds,
    } = warp_bounds(offset, size, warp)?;
    if max_edge.is_none_or(|edge| size.0.max(size.1) <= edge) {
        let exact = match warp {
            Warp::Flip(mirror) => Some(mirror_pixels(pixels, mirror)),
            warp if identity_warp(offset, size, warp) => Some(pixels.clone()),
            Warp::Rotate(degrees) => exact_turn(degrees).map(|turn| turn_pixels(pixels, turn)),
            Warp::RotateAround { degrees, pivot } => exact_turn(degrees)
                .filter(|turn| {
                    bounds == turned_size(size.0, size.1, *turn)
                        && rotate_corners_about(layer_corners(offset, size), degrees, pivot)
                            .iter()
                            .all(|point| {
                                (point.0 - point.0.round()).abs() < 1e-4
                                    && (point.1 - point.1.round()).abs() < 1e-4
                            })
                })
                .map(|turn| turn_pixels(pixels, turn)),
            _ => None,
        };
        if let Some(pixels) = exact {
            return Ok(Placed {
                pixels,
                offset: placed_offset,
                bounds,
            });
        }
    }
    let grid = output_grid(bounds, max_edge);
    let mut image = RgbaImage::new(grid.0, grid.1);
    let source = layer_corners(offset, size);
    let dest = match warp {
        Warp::Rotate(degrees) => placed_rotation_outline(offset, size, degrees),
        Warp::RotateAround { degrees, pivot } => rotate_corners_about(source, degrees, pivot),
        Warp::Perspective(corners) => corners,
        Warp::Flip(_) => source,
    };
    let mapping = QuadMapping::new(dest).ok_or(WarpError::Singular)?;
    for y in 0..grid.1 {
        for x in 0..grid.0 {
            let canvas_x = f64::from(placed_offset.0)
                + (f64::from(x) + 0.5) * f64::from(bounds.0) / f64::from(grid.0);
            let canvas_y = f64::from(placed_offset.1)
                + (f64::from(y) + 0.5) * f64::from(bounds.1) / f64::from(grid.1);
            let Some((mut u, mut v)) = mapping.unmap((canvas_x, canvas_y)) else {
                continue;
            };
            if let Warp::Flip(mirror) = warp {
                match mirror {
                    Mirror::Horizontal => u = 1.0 - u,
                    Mirror::Vertical => v = 1.0 - v,
                }
            }
            let sample_x = u * f64::from(size.0) - 0.5;
            let sample_y = v * f64::from(size.1) - 0.5;
            image.put_pixel(x, y, sample_pixel(pixels, sample_x, sample_y, resample));
        }
    }
    Ok(Placed {
        pixels: image,
        offset: placed_offset,
        bounds,
    })
}

// Homography from the unit square to the destination quad. Using coordinates
// relative to its first corner keeps the solve stable at large layer offsets.
struct QuadMapping {
    origin: (f64, f64),
    u: (f64, f64),
    v: (f64, f64),
    perspective: (f64, f64),
}

impl QuadMapping {
    fn new(corners: [(f64, f64); 4]) -> Option<Self> {
        let [origin, p1, p2, p3] = corners;
        let subtract = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0, a.1 - b.1);
        let a = subtract(p1, p2);
        let b = subtract(p3, p2);
        let p1 = subtract(p1, origin);
        let p2 = subtract(p2, origin);
        let p3 = subtract(p3, origin);
        let delta = (p2.0 - p1.0 - p3.0, p2.1 - p1.1 - p3.1);
        let det = cross(a, b);
        if det.abs() < 1e-8 {
            return None;
        }
        let g = cross(delta, b) / det;
        let h = cross(a, delta) / det;
        Some(Self {
            origin,
            u: (p1.0 * (1.0 + g), p1.1 * (1.0 + g)),
            v: (p3.0 * (1.0 + h), p3.1 * (1.0 + h)),
            perspective: (g, h),
        })
    }

    fn unmap(&self, point: (f64, f64)) -> Option<(f64, f64)> {
        let (x, y) = (point.0 - self.origin.0, point.1 - self.origin.1);
        let (g, h) = self.perspective;
        let a = (self.u.0 - x * g, self.u.1 - y * g);
        let b = (self.v.0 - x * h, self.v.1 - y * h);
        let det = cross(a, b);
        if det.abs() < 1e-8 {
            return None;
        }
        let u = cross((x, y), b) / det;
        let v = cross(a, (x, y)) / det;
        let inside = -1e-8..=1.0 + 1e-8;
        (inside.contains(&u) && inside.contains(&v)).then(|| (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)))
    }
}

fn identity_warp(offset: (i32, i32), size: (u32, u32), warp: Warp) -> bool {
    match warp {
        Warp::Rotate(degrees) => {
            let turns = degrees / 360.0;
            (turns - turns.round()).abs() < 1e-6
        }
        Warp::RotateAround { degrees, .. } => degrees.rem_euclid(360.) == 0.,
        Warp::Flip(_) => false,
        Warp::Perspective(corners) => corners
            .iter()
            .zip(layer_corners(offset, size))
            .all(|(corner, source)| (corner.0 - source.0).hypot(corner.1 - source.1) < 1e-3),
    }
}

fn exact_turn(degrees: f64) -> Option<Turn> {
    let degrees = degrees.rem_euclid(360.0);
    [
        (90.0, Turn::Right),
        (180.0, Turn::Half),
        (270.0, Turn::Left),
    ]
    .into_iter()
    .find_map(|(angle, turn)| ((degrees - angle).abs() < 1e-6).then_some(turn))
}

pub(crate) fn placed_rotation_outline(
    offset: (i32, i32),
    size: (u32, u32),
    degrees: f64,
) -> [(f64, f64); 4] {
    let mut corners = rotate_corners(layer_corners(offset, size), degrees);
    if exact_turn(degrees).is_some() {
        let placed = warp_bounds(offset, size, Warp::Rotate(degrees)).unwrap();
        let left = corners.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let top = corners.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        for (x, y) in &mut corners {
            *x += f64::from(placed.offset.0) - left;
            *y += f64::from(placed.offset.1) - top;
        }
    }
    corners
}

pub(crate) fn rotate_corners(corners: [(f64, f64); 4], degrees: f64) -> [(f64, f64); 4] {
    let center = (
        (corners[0].0 + corners[2].0) / 2.0,
        (corners[0].1 + corners[2].1) / 2.0,
    );
    let (sin, cos) = degrees.to_radians().sin_cos();
    corners.map(|corner| {
        let dx = corner.0 - center.0;
        let dy = corner.1 - center.1;
        (
            center.0 + dx * cos - dy * sin,
            center.1 + dx * sin + dy * cos,
        )
    })
}

pub(crate) fn rotate_corners_about(
    corners: [(f64, f64); 4],
    degrees: f64,
    pivot: (f64, f64),
) -> [(f64, f64); 4] {
    if degrees.rem_euclid(360.) == 0. {
        return corners;
    }
    let (sin, cos) = match exact_turn(degrees) {
        Some(Turn::Right) => (1., 0.),
        Some(Turn::Half) => (0., -1.),
        Some(Turn::Left) => (-1., 0.),
        None => degrees.rem_euclid(360.).to_radians().sin_cos(),
    };
    corners.map(|(x, y)| {
        let (dx, dy) = (x - pivot.0, y - pivot.1);
        (pivot.0 + dx * cos - dy * sin, pivot.1 + dx * sin + dy * cos)
    })
}

fn validate_quad(corners: [(f64, f64); 4]) -> Result<(), WarpError> {
    if corners
        .iter()
        .any(|corner| !corner.0.is_finite() || !corner.1.is_finite())
    {
        return Err(WarpError::Singular);
    }
    for index in 0..4 {
        let edge = corners[index];
        let next = corners[(index + 1) % 4];
        if (next.0 - edge.0).hypot(next.1 - edge.1) < 1e-3 {
            return Err(WarpError::Singular);
        }
    }
    if segments_cross(corners[0], corners[1], corners[2], corners[3])
        || segments_cross(corners[1], corners[2], corners[3], corners[0])
    {
        return Err(WarpError::Crossing);
    }
    let turns = std::array::from_fn::<_, 4, _>(|i| {
        orient(corners[i], corners[(i + 1) % 4], corners[(i + 2) % 4])
    });
    if turns.iter().any(|turn| turn.abs() < 1e-6) {
        return Err(WarpError::Singular);
    }
    if turns.iter().any(|turn| turn.signum() != turns[0].signum()) {
        return Err(WarpError::Concave);
    }
    let area = (orient(corners[0], corners[1], corners[2])
        + orient(corners[0], corners[2], corners[3]))
    .abs()
        * 0.5;
    if area < 0.5 {
        return Err(WarpError::Singular);
    }
    Ok(())
}

fn segments_cross(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> bool {
    let first = orient(a, b, c) * orient(a, b, d);
    let second = orient(c, d, a) * orient(c, d, b);
    first < 0.0 && second < 0.0
}

fn orient(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn bounds_from_corners(corners: [(f64, f64); 4]) -> Result<PlacedBox, WarpError> {
    let min_x = corners
        .iter()
        .map(|corner| corner.0)
        .fold(f64::INFINITY, f64::min);
    let min_y = corners
        .iter()
        .map(|corner| corner.1)
        .fold(f64::INFINITY, f64::min);
    let max_x = corners
        .iter()
        .map(|corner| corner.0)
        .fold(f64::NEG_INFINITY, f64::max);
    let max_y = corners
        .iter()
        .map(|corner| corner.1)
        .fold(f64::NEG_INFINITY, f64::max);
    let x = snap_min(min_x).ok_or(WarpError::TooLarge)?;
    let y = snap_min(min_y).ok_or(WarpError::TooLarge)?;
    let width = span_u32(max_x - f64::from(x)).ok_or(WarpError::TooLarge)?;
    let height = span_u32(max_y - f64::from(y)).ok_or(WarpError::TooLarge)?;
    if !canvas_allowed(width, height) {
        return Err(WarpError::TooLarge);
    }
    Ok(PlacedBox {
        offset: (x, y),
        size: (width, height),
    })
}

fn snap_min(value: f64) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }
    let nearest = value.round();
    let snapped = if (value - nearest).abs() < 1e-4 {
        nearest
    } else {
        value.floor()
    };
    as_i32(snapped)
}

fn span_u32(length: f64) -> Option<u32> {
    if !length.is_finite() || length <= 0.0 {
        return None;
    }
    let nearest = length.round();
    let snapped = if (length - nearest).abs() < 1e-4 {
        nearest
    } else {
        length.ceil()
    };
    if !(1.0..=f64::from(MAX_NEW_EDGE)).contains(&snapped) {
        return None;
    }
    Some(snapped as u32)
}

fn as_i32(value: f64) -> Option<i32> {
    if value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return None;
    }
    Some(value as i32)
}

fn output_grid(bounds: (u32, u32), max_edge: Option<u32>) -> (u32, u32) {
    let Some(max_edge) = max_edge else {
        return bounds;
    };
    let long = bounds.0.max(bounds.1);
    if long <= max_edge || long == 0 {
        return bounds;
    }
    let scale = f64::from(max_edge) / f64::from(long);
    (
        (f64::from(bounds.0) * scale).round().max(1.0) as u32,
        (f64::from(bounds.1) * scale).round().max(1.0) as u32,
    )
}

fn cross(left: (f64, f64), right: (f64, f64)) -> f64 {
    left.0 * right.1 - left.1 * right.0
}

fn sample_pixel(pixels: &RgbaImage, x: f64, y: f64, resample: Resample) -> Rgba<u8> {
    if !x.is_finite() || !y.is_finite() {
        return Rgba([0, 0, 0, 0]);
    }
    match resample {
        Resample::Nearest => {
            let sx = x.round();
            let sy = y.round();
            pixel_at(pixels, sx as i64, sy as i64).unwrap_or(Rgba([0, 0, 0, 0]))
        }
        Resample::Bilinear => unpremultiply_sample(bilinear(pixels, x, y)),
        Resample::Bicubic => unpremultiply_sample(bicubic(pixels, x, y)),
    }
}

fn bilinear(pixels: &RgbaImage, x: f64, y: f64) -> [f64; 4] {
    let x0 = x.floor();
    let y0 = y.floor();
    let tx = x - x0;
    let ty = y - y0;
    let mut mixed = [0.0; 4];
    for (iy, wy) in [(y0, 1.0 - ty), (y0 + 1.0, ty)] {
        for (ix, wx) in [(x0, 1.0 - tx), (x0 + 1.0, tx)] {
            let weight = wx * wy;
            let sample = premul(pixel_at(pixels, ix as i64, iy as i64));
            for (slot, sample) in mixed.iter_mut().zip(sample) {
                *slot += sample * weight;
            }
        }
    }
    mixed
}

fn bicubic(pixels: &RgbaImage, x: f64, y: f64) -> [f64; 4] {
    let x1 = x.floor();
    let y1 = y.floor();
    let tx = x - x1;
    let ty = y - y1;
    let mut rows = [[0.0; 4]; 4];
    for (row, iy) in (-1..=2).enumerate() {
        let samples = [
            premul(pixel_at(pixels, (x1 - 1.0) as i64, (y1 + iy as f64) as i64)),
            premul(pixel_at(pixels, x1 as i64, (y1 + iy as f64) as i64)),
            premul(pixel_at(pixels, (x1 + 1.0) as i64, (y1 + iy as f64) as i64)),
            premul(pixel_at(pixels, (x1 + 2.0) as i64, (y1 + iy as f64) as i64)),
        ];
        for (slot, channel) in rows[row].iter_mut().zip(0..4) {
            *slot = catmull(
                [
                    samples[0][channel],
                    samples[1][channel],
                    samples[2][channel],
                    samples[3][channel],
                ],
                tx,
            );
        }
    }
    let mut mixed = [0.0; 4];
    for (slot, channel) in mixed.iter_mut().zip(0..4) {
        *slot = catmull(
            [
                rows[0][channel],
                rows[1][channel],
                rows[2][channel],
                rows[3][channel],
            ],
            ty,
        );
    }
    mixed
}

fn catmull(samples: [f64; 4], t: f64) -> f64 {
    let a = -0.5 * samples[0] + 1.5 * samples[1] - 1.5 * samples[2] + 0.5 * samples[3];
    let b = samples[0] - 2.5 * samples[1] + 2.0 * samples[2] - 0.5 * samples[3];
    let c = -0.5 * samples[0] + 0.5 * samples[2];
    ((a * t + b) * t + c) * t + samples[1]
}

fn pixel_at(pixels: &RgbaImage, x: i64, y: i64) -> Option<Rgba<u8>> {
    let x = u32::try_from(x).ok()?;
    let y = u32::try_from(y).ok()?;
    (x < pixels.width() && y < pixels.height()).then(|| *pixels.get_pixel(x, y))
}

fn premul(pixel: Option<Rgba<u8>>) -> [f64; 4] {
    let Some(pixel) = pixel else {
        return [0.0; 4];
    };
    let alpha = f64::from(pixel.0[3]) / 255.0;
    [
        f64::from(pixel.0[0]) / 255.0 * alpha,
        f64::from(pixel.0[1]) / 255.0 * alpha,
        f64::from(pixel.0[2]) / 255.0 * alpha,
        alpha,
    ]
}

fn unpremultiply_sample(sample: [f64; 4]) -> Rgba<u8> {
    let alpha = sample[3].clamp(0.0, 1.0);
    if alpha <= 0.0 {
        return Rgba([0, 0, 0, 0]);
    }
    let scale = 255.0 / alpha;
    let byte = |value: f64| (value.clamp(0.0, alpha) * scale).round() as u8;
    Rgba([
        byte(sample[0]),
        byte(sample[1]),
        byte(sample[2]),
        (alpha * 255.0).round() as u8,
    ])
}

type PremultipliedImage = ImageBuffer<Rgba<u16>, Vec<u16>>;

fn premultiply(pixels: &RgbaImage) -> PremultipliedImage {
    // Keep every 8-bit color × alpha product exact. Rounding these products to
    // 8 bits before filtering can turn a faint colored pixel into a different hue.
    ImageBuffer::from_fn(pixels.width(), pixels.height(), |x, y| {
        let pixel = pixels.get_pixel(x, y);
        let alpha = u16::from(pixel[3]);
        Rgba([
            u16::from(pixel[0]) * alpha,
            u16::from(pixel[1]) * alpha,
            u16::from(pixel[2]) * alpha,
            alpha * 255,
        ])
    })
}

fn unpremultiply(pixels: &PremultipliedImage) -> RgbaImage {
    RgbaImage::from_fn(pixels.width(), pixels.height(), |x, y| {
        let pixel = pixels.get_pixel(x, y);
        let alpha = u32::from(pixel[3]);
        let output_alpha = ((alpha + 127) / 255).min(255) as u8;
        if output_alpha == 0 {
            return Rgba([0; 4]);
        }
        let channel = |value: u16| ((u32::from(value).min(alpha) * 255 + alpha / 2) / alpha) as u8;
        Rgba([
            channel(pixel[0]),
            channel(pixel[1]),
            channel(pixel[2]),
            output_alpha,
        ])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row() -> RgbaImage {
        let mut image = RgbaImage::new(3, 1);
        image.put_pixel(0, 0, Rgba([1, 0, 0, 255]));
        image.put_pixel(1, 0, Rgba([2, 0, 0, 128]));
        image.put_pixel(2, 0, Rgba([3, 0, 0, 0]));
        image
    }

    #[test]
    fn nearest_scale_duplicates_pixels_and_edges_scale_with_the_canvas() {
        let scaled = scale_pixels(&row(), 6, 1, Resample::Nearest);
        assert_eq!(scaled.get_pixel(0, 0).0, [1, 0, 0, 255]);
        assert_eq!(scaled.get_pixel(2, 0).0, [2, 0, 0, 128]);
        assert_eq!(scaled.get_pixel(5, 0).0, [3, 0, 0, 0]);
        assert_eq!(
            scale_placement((-1, 0), (2, 1), (4, 2), (8, 2)),
            Some(((-2, 0), (4, 1)))
        );
        assert_eq!(
            scale_rect(PixelRect::new(1, 0, 2, 1), (4, 2), (8, 4)),
            PixelRect::new(2, 0, 4, 2)
        );
    }

    #[test]
    fn bilinear_scale_keeps_the_color_of_a_transparent_edge() {
        let mut image = RgbaImage::new(2, 1);
        image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        image.put_pixel(1, 0, Rgba([0, 0, 0, 0]));
        let scaled = scale_pixels(&image, 3, 1, Resample::Bilinear);
        let middle = scaled.get_pixel(1, 0).0;
        assert!(middle[3] > 0 && middle[3] < 255);
        let red = u16::from(middle[0]) * 255 / u16::from(middle[3]);
        assert!(red > 200, "{middle:?}");
        assert_eq!(middle[1], 0);
    }
}
