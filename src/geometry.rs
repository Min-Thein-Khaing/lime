use image::RgbaImage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentCrop {
    // Every sampled pixel is transparent or outside the canvas.
    Empty,
    // The content already fills the layer or the canvas.
    Unchanged,
    Crop(PixelBox),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
    HorizontalCenter,
    VerticalCenter,
    Both,
}

pub struct LayerSample<'a> {
    pub offset: (i32, i32),
    pub pixels: &'a RgbaImage,
    pub visible: bool,
    pub opacity: u8,
}

pub fn aligned_offset(
    offset: (i32, i32),
    layer_width: u32,
    layer_height: u32,
    canvas_width: u32,
    canvas_height: u32,
    alignment: Alignment,
) -> (i32, i32) {
    let clamp = |value: i64| value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
    let center = |canvas: u32, layer: u32| clamp((i64::from(canvas) - i64::from(layer)) / 2);
    let edge = |canvas: u32, layer: u32| clamp(i64::from(canvas) - i64::from(layer));
    match alignment {
        Alignment::Left => (0, offset.1),
        Alignment::Right => (edge(canvas_width, layer_width), offset.1),
        Alignment::Top => (offset.0, 0),
        Alignment::Bottom => (offset.0, edge(canvas_height, layer_height)),
        Alignment::HorizontalCenter => (center(canvas_width, layer_width), offset.1),
        Alignment::VerticalCenter => (offset.0, center(canvas_height, layer_height)),
        Alignment::Both => (
            center(canvas_width, layer_width),
            center(canvas_height, layer_height),
        ),
    }
}

// Fits the whole layer, including transparent margins, inside the canvas without
// cropping. The limiting edge is exact; the other rounds to the nearest pixel.
pub fn proportional_fit(layer: (u32, u32), canvas: (u32, u32)) -> Option<PixelBox> {
    let (width, height) = layer;
    let (cw, ch) = canvas;
    if width == 0 || height == 0 || cw == 0 || ch == 0 {
        return None;
    }
    let rounded = |size: u32, edge: u32, divisor: u32, limit: u32| {
        ((u64::from(size) * u64::from(edge) + u64::from(divisor) / 2) / u64::from(divisor))
            .clamp(1, u64::from(limit)) as u32
    };
    let (width, height) = if u64::from(cw) * u64::from(height) <= u64::from(ch) * u64::from(width) {
        (cw, rounded(height, cw, width, ch))
    } else {
        (rounded(width, ch, height, cw), ch)
    };
    Some(PixelBox {
        x: (cw - width) / 2,
        y: (ch - height) / 2,
        width,
        height,
    })
}

pub fn nonzero_alpha_bounds(pixels: &RgbaImage) -> Option<PixelBox> {
    let (width, height) = pixels.dimensions();
    let (mut min_x, mut min_y) = (width, height);
    let (mut max_x, mut max_y) = (0, 0);
    let mut found = false;
    for (x, y, pixel) in pixels.enumerate_pixels() {
        if pixel.0[3] == 0 {
            continue;
        }
        found = true;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    found.then(|| PixelBox {
        x: min_x,
        y: min_y,
        width: max_x - min_x + 1,
        height: max_y - min_y + 1,
    })
}

pub fn layer_content_crop(pixels: &RgbaImage) -> ContentCrop {
    let Some(bounds) = nonzero_alpha_bounds(pixels) else {
        return ContentCrop::Empty;
    };
    let (width, height) = pixels.dimensions();
    if bounds.x == 0 && bounds.y == 0 && bounds.width == width && bounds.height == height {
        ContentCrop::Unchanged
    } else {
        ContentCrop::Crop(bounds)
    }
}

// Copies the part of the layer that already lies on the canvas into a canvas-sized image.
// Pixels outside the canvas are dropped. An exact fit at the origin needs no change.
pub fn layer_to_image_size(
    offset: (i32, i32),
    pixels: &RgbaImage,
    canvas_width: u32,
    canvas_height: u32,
) -> Option<RgbaImage> {
    if offset == (0, 0) && pixels.dimensions() == (canvas_width, canvas_height) {
        return None;
    }
    let mut image = RgbaImage::new(canvas_width, canvas_height);
    let (width, height) = pixels.dimensions();
    for y in 0..height {
        for x in 0..width {
            let canvas_x = i64::from(offset.0) + i64::from(x);
            let canvas_y = i64::from(offset.1) + i64::from(y);
            if canvas_x < 0
                || canvas_y < 0
                || canvas_x >= i64::from(canvas_width)
                || canvas_y >= i64::from(canvas_height)
            {
                continue;
            }
            image.put_pixel(canvas_x as u32, canvas_y as u32, *pixels.get_pixel(x, y));
        }
    }
    Some(image)
}

// The canvas rectangle covering visible, nonzero-alpha pixels. Hidden layers and
// pixels that sit off the canvas do not enlarge it.
pub fn image_content_crop(layers: &[LayerSample<'_>], width: u32, height: u32) -> ContentCrop {
    let (mut min_x, mut min_y) = (width, height);
    let (mut max_x, mut max_y) = (0, 0);
    let mut found = false;
    for layer in layers {
        if !layer.visible || layer.opacity == 0 {
            continue;
        }
        let (layer_width, layer_height) = layer.pixels.dimensions();
        for y in 0..layer_height {
            for x in 0..layer_width {
                if layer.pixels.get_pixel(x, y).0[3] == 0 {
                    continue;
                }
                let canvas_x = i64::from(layer.offset.0) + i64::from(x);
                let canvas_y = i64::from(layer.offset.1) + i64::from(y);
                if canvas_x < 0
                    || canvas_y < 0
                    || canvas_x >= i64::from(width)
                    || canvas_y >= i64::from(height)
                {
                    continue;
                }
                let (canvas_x, canvas_y) = (canvas_x as u32, canvas_y as u32);
                found = true;
                min_x = min_x.min(canvas_x);
                min_y = min_y.min(canvas_y);
                max_x = max_x.max(canvas_x);
                max_y = max_y.max(canvas_y);
            }
        }
    }
    if !found {
        return ContentCrop::Empty;
    }
    let bounds = PixelBox {
        x: min_x,
        y: min_y,
        width: max_x - min_x + 1,
        height: max_y - min_y + 1,
    };
    if bounds.x == 0 && bounds.y == 0 && bounds.width == width && bounds.height == height {
        ContentCrop::Unchanged
    } else {
        ContentCrop::Crop(bounds)
    }
}
