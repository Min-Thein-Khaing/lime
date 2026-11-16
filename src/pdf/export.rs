//! Single-page raster PDF export. DPI changes physical size, never pixel count.

use std::io::Write;

use anyhow::{Result, ensure};
use flate2::{Compression, write::ZlibEncoder};
use image::RgbaImage;
use pdf_writer::{Content, Filter, Finish, Name, Pdf, Rect, Ref};

pub const DEFAULT_DPI: u32 = 300;
pub const MIN_DPI: u32 = 1;
pub const MAX_DPI: u32 = 1200;

pub fn page_size(size: (u32, u32), dpi: u32) -> Result<(f32, f32)> {
    ensure!(
        (MIN_DPI..=MAX_DPI).contains(&dpi),
        "Enter a resolution from {MIN_DPI} to {MAX_DPI} DPI."
    );
    ensure!(size.0 > 0 && size.1 > 0, "The canvas is empty.");
    // Keep within the standard 200-inch page limit without viewer-specific scaling.
    let minimum = size.0.max(size.1).div_ceil(200);
    ensure!(
        dpi >= minimum,
        "This canvas needs at least {minimum} DPI to fit within the PDF page size limit."
    );
    Ok((
        size.0 as f32 * 72. / dpi as f32,
        size.1 as f32 * 72. / dpi as f32,
    ))
}

pub fn write(mut writer: impl Write, pixels: &RgbaImage, dpi: u32) -> Result<()> {
    let (width, height) = page_size(pixels.dimensions(), dpi)?;
    // Compress one RGB row at a time instead of allocating another full raster.
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    let mut row = Vec::with_capacity(pixels.width() as usize * 3);
    for pixels in pixels.rows() {
        row.clear();
        for pixel in pixels {
            row.extend_from_slice(&crate::document::rgb_over_white(*pixel));
        }
        encoder.write_all(&row)?;
    }
    let encoded = encoder.finish()?;
    let mut pdf = Pdf::new();
    let catalog = Ref::new(1);
    let pages = Ref::new(2);
    let page = Ref::new(3);
    let image = Ref::new(4);
    let contents = Ref::new(5);
    let profile = Ref::new(6);
    let image_name = Name(b"Canvas");
    pdf.catalog(catalog).pages(pages);
    pdf.pages(pages).kids([page]).count(1);
    let mut page_writer = pdf.page(page);
    page_writer.parent(pages);
    page_writer.media_box(Rect::new(0., 0., width, height));
    page_writer.contents(contents);
    page_writer.resources().x_objects().pair(image_name, image);
    page_writer.finish();

    let mut image_writer = pdf.image_xobject(image, &encoded);
    image_writer.width(pixels.width().try_into()?);
    image_writer.height(pixels.height().try_into()?);
    image_writer.bits_per_component(8);
    image_writer.color_space().icc_based(profile);
    image_writer.filter(Filter::FlateDecode);
    image_writer.interpolate(false);
    image_writer.finish();
    let srgb = crate::color_management::srgb_profile()?;
    let mut icc = pdf.icc_profile(profile, &srgb);
    icc.n(3);
    icc.alternate().device_rgb();
    icc.finish();

    let mut content = Content::new();
    content.save_state();
    content.transform([width, 0., 0., height, 0., 0.]);
    content.x_object(image_name);
    content.restore_state();
    pdf.stream(contents, &content.finish());
    writer.write_all(&pdf.finish())?;
    Ok(())
}
