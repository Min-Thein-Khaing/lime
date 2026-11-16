use std::{
    fs::File,
    io::Read,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{Context as _, Result, anyhow, ensure};
use hayro::{
    PixmapSettings, RenderCache, RenderSettings,
    hayro_interpret::InterpreterSettings,
    hayro_syntax::{Pdf, PdfData},
    render,
    vello_cpu::{
        color::palette::css::{TRANSPARENT, WHITE},
        peniko::ImageAlphaType,
    },
};
use image::RgbaImage;

use crate::document::{Document, MAX_STACK_BYTES};

pub mod export;
mod pages;
mod worker;
pub use worker::run_if_requested;
mod thumbnails;
pub use pages::{
    MAX_PAGE_SELECTION_CHARS, parse_page_selection, toggle_page_selection, validate_page_numbers,
};
pub use thumbnails::{THUMBNAIL_BATCH, render_thumbnails};

pub const MAX_PDF_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_PDF_PAGES: usize = 10_000;
pub const DEFAULT_DPI: u32 = 150;
pub const MIN_DPI: u32 = 1;
pub const MAX_DPI: u32 = 1200;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PdfMode {
    Layers,
    Separate,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PdfBackground {
    #[default]
    White,
    Transparent,
}

impl PdfBackground {
    fn flatten_background(self, image: &mut RgbaImage) {
        if self == Self::White {
            // Renderer rounding at antialiased edges can leave alpha at 254
            // even over its white backdrop. White output must stay opaque.
            for pixel in image.pixels_mut() {
                if pixel[3] != 255 {
                    *pixel = crate::document::source_over(image::Rgba([255; 4]), *pixel, 255);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PdfCanvas {
    #[default]
    FirstPage,
    FitAllPages,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PdfInfo {
    pub pages: usize,
    pub sizes: Vec<(f32, f32)>,
}

#[derive(Clone, Debug)]
pub struct PdfSource {
    data: PdfData,
    render_lock: Arc<Mutex<()>>,
    pub info: PdfInfo,
}

#[derive(Clone, Default)]
pub struct PdfControl(Arc<AtomicBool>);

#[derive(Debug)]
pub struct PdfCanceled;

impl std::fmt::Display for PdfCanceled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PDF import canceled.")
    }
}
impl std::error::Error for PdfCanceled {}

impl PdfControl {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn canceled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    pub fn check(&self) -> Result<()> {
        if self.canceled() {
            Err(PdfCanceled.into())
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PdfProgress {
    Reading,
    Inspecting,
    Rendering {
        page: usize,
        done: usize,
        total: usize,
    },
    Preparing {
        done: usize,
        total: usize,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PdfOptions {
    pub path: PathBuf,
    pub pages: Vec<usize>,
    pub dpi: u32,
    pub mode: PdfMode,
    pub background: PdfBackground,
    pub canvas: PdfCanvas,
}

pub fn layered_canvas_size(sizes: &[(u32, u32)], policy: PdfCanvas) -> Result<(u32, u32)> {
    let mut canvas = *sizes
        .first()
        .ok_or_else(|| anyhow!("Choose at least one page."))?;
    if policy == PdfCanvas::FitAllPages {
        for size in sizes {
            canvas = (canvas.0.max(size.0), canvas.1.max(size.1));
        }
    }
    Document::validate_size(canvas.0, canvas.1).context(
        "The layered canvas is too large. Use First Page, lower the DPI, or choose fewer pages",
    )?;
    Ok(canvas)
}

pub fn is_pdf_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

pub fn read_source(
    path: &Path,
    control: &PdfControl,
    progress: impl Fn(PdfProgress),
) -> Result<PdfSource> {
    control.check()?;
    progress(PdfProgress::Reading);
    control.check()?;
    let bytes = read_pdf(path, control)?;
    let data = PdfData::from(bytes);
    progress(PdfProgress::Inspecting);
    control.check()?;
    catch_unwind(AssertUnwindSafe(|| {
        let info = if worker::enabled() {
            worker::inspect(&data, control)?
        } else {
            let pdf = load_pdf(data.clone())?;
            info_from_pdf(&pdf, control)?
        };
        control.check()?;
        Ok(PdfSource {
            data,
            info,
            render_lock: Default::default(),
        })
    }))
    .map_err(|_| anyhow!("The PDF could not be read."))?
}

impl PdfSource {
    fn acquire(&self, control: &PdfControl) -> Result<std::sync::MutexGuard<'_, ()>> {
        loop {
            control.check()?;
            match self.render_lock.try_lock() {
                Ok(guard) => return Ok(guard),
                Err(std::sync::TryLockError::Poisoned(_)) => {
                    return Err(anyhow!("The PDF renderer is unavailable."));
                }
                Err(std::sync::TryLockError::WouldBlock) => {
                    std::thread::sleep(std::time::Duration::from_millis(20))
                }
            }
        }
    }
}

pub fn render_pages(
    source: &PdfSource,
    options: &PdfOptions,
    control: &PdfControl,
    progress: impl Fn(PdfProgress),
) -> Result<Vec<RgbaImage>> {
    control.check()?;
    let _guard = source.acquire(control)?;
    control.check()?;
    catch_unwind(AssertUnwindSafe(|| {
        if worker::enabled() {
            worker::pages(source, options, control, &progress)
        } else {
            render_data(source.data.clone(), options, control, &progress)
        }
    }))
    .map_err(|_| anyhow!("The PDF could not be rendered."))?
}

pub fn scaled_page_size(size: (f32, f32), dpi: u32) -> (u32, u32) {
    // Match hayro's f32 scale and truncated integer pixmap dimensions exactly.
    let scale = dpi as f32 / 72.0;
    ((size.0 * scale) as u32, (size.1 * scale) as u32)
}

pub fn validate_options(info: &PdfInfo, options: &PdfOptions) -> Result<Vec<(u32, u32)>> {
    ensure!(!info.sizes.is_empty(), "The PDF has no pages.");
    ensure!(
        info.pages == info.sizes.len() && info.pages <= MAX_PDF_PAGES,
        "The PDF page information is invalid."
    );
    ensure!(
        (MIN_DPI..=MAX_DPI).contains(&options.dpi),
        "Enter a resolution from {MIN_DPI} to {MAX_DPI} DPI."
    );
    validate_page_numbers(&options.pages, info.pages)?;
    let mut sizes = Vec::with_capacity(options.pages.len());
    let mut total_bytes = 0u64;
    for &page in &options.pages {
        let index = page - 1;
        ensure!(
            info.sizes[index].0.is_finite()
                && info.sizes[index].1.is_finite()
                && info.sizes[index].0 > 0.
                && info.sizes[index].1 > 0.,
            "Page {} has invalid dimensions.",
            index + 1
        );
        let size = scaled_page_size(info.sizes[index], options.dpi);
        ensure!(
            (1..=u32::from(u16::MAX)).contains(&size.0)
                && (1..=u32::from(u16::MAX)).contains(&size.1),
            "Page {page} at {} DPI is outside the renderer's pixel limits. Choose another resolution.",
            options.dpi
        );
        Document::validate_size(size.0, size.1)
            .with_context(|| format!("Page {page} at {} DPI is too large", options.dpi))?;
        let bytes = u64::from(size.0)
            .saturating_mul(u64::from(size.1))
            .saturating_mul(4);
        total_bytes = total_bytes.saturating_add(bytes);
        ensure!(
            total_bytes <= MAX_STACK_BYTES,
            "The selected pages need too much memory. Use a lower resolution or fewer pages."
        );
        sizes.push(size);
    }
    if options.mode == PdfMode::Layers {
        layered_canvas_size(&sizes, options.canvas)?;
    }
    Ok(sizes)
}

fn read_pdf(path: &Path, control: &PdfControl) -> Result<Vec<u8>> {
    control.check()?;
    let path = path.canonicalize().context("The file could not be found")?;
    ensure!(path.is_file(), "Choose a PDF file");
    let metadata = std::fs::metadata(&path).context("The file could not be opened")?;
    ensure!(
        metadata.len() <= MAX_PDF_BYTES,
        "The PDF is larger than {} MB.",
        MAX_PDF_BYTES / (1024 * 1024)
    );
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    let mut file = File::open(&path)
        .context("The file could not be opened")?
        .take(MAX_PDF_BYTES + 1);
    let mut chunk = [0u8; 64 * 1024];
    loop {
        control.check()?;
        let read = file.read(&mut chunk).context("The PDF could not be read")?;
        if read == 0 {
            break;
        }
        ensure!(
            bytes.len() as u64 + read as u64 <= MAX_PDF_BYTES,
            "The PDF is larger than {} MB.",
            MAX_PDF_BYTES / (1024 * 1024)
        );
        bytes.extend_from_slice(&chunk[..read]);
    }
    control.check()?;
    ensure!(
        bytes.len() as u64 <= MAX_PDF_BYTES,
        "The PDF is larger than {} MB.",
        MAX_PDF_BYTES / (1024 * 1024)
    );
    ensure!(
        bytes.windows(5).take(1024).any(|window| window == b"%PDF-"),
        "Choose a PDF file"
    );
    Ok(bytes)
}

fn load_pdf(bytes: impl Into<PdfData>) -> Result<Pdf> {
    let pdf = catch_unwind(AssertUnwindSafe(|| Pdf::new(bytes)))
        .map_err(|_| anyhow!("The PDF could not be read."))?;
    pdf.map_err(|error| {
        let detail = format!("{error:?}");
        let lower = detail.to_ascii_lowercase();
        if lower.contains("encrypt") || lower.contains("decrypt") || lower.contains("password") {
            anyhow!("Encrypted PDFs are not supported yet.")
        } else {
            anyhow!("The PDF could not be read: {detail}")
        }
    })
}

fn info_from_pdf(pdf: &Pdf, control: &PdfControl) -> Result<PdfInfo> {
    control.check()?;
    let pages = pdf.pages();
    control.check()?;
    ensure!(!pages.is_empty(), "The PDF has no pages.");
    ensure!(
        pages.len() <= MAX_PDF_PAGES,
        "The PDF has more than {MAX_PDF_PAGES} pages."
    );
    Ok(PdfInfo {
        pages: pages.len(),
        sizes: pages
            .iter()
            .map(|page| {
                control.check()?;
                Ok(page.render_dimensions())
            })
            .collect::<Result<_>>()?,
    })
}

fn render_data(
    data: PdfData,
    options: &PdfOptions,
    control: &PdfControl,
    progress: &impl Fn(PdfProgress),
) -> Result<Vec<RgbaImage>> {
    let mut pages = Vec::new();
    render_each(data, options, control, progress, |_, image| {
        pages.push(image);
        Ok(())
    })?;
    Ok(pages)
}

fn render_each(
    data: PdfData,
    options: &PdfOptions,
    control: &PdfControl,
    progress: &impl Fn(PdfProgress),
    mut emit: impl FnMut(usize, RgbaImage) -> Result<()>,
) -> Result<()> {
    control.check()?;
    progress(PdfProgress::Inspecting);
    control.check()?;
    let pdf = load_pdf(data)?;
    let info = info_from_pdf(&pdf, control)?;
    let expected = validate_options(&info, options)?;
    let pages = pdf.pages();
    let cache = RenderCache::new();
    let render_settings = RenderSettings::default();
    let interpreter_settings = InterpreterSettings::default();
    let scale = options.dpi as f32 / 72.0;
    let mut total_bytes = 0u64;

    for (index, &page_number) in options.pages.iter().enumerate() {
        let page = &pages[page_number - 1];
        control.check()?;
        progress(PdfProgress::Rendering {
            page: page_number,
            done: index,
            total: expected.len(),
        });
        control.check()?;
        let pixmap = catch_unwind(AssertUnwindSafe(|| {
            render(
                page,
                &cache,
                &interpreter_settings,
                &render_settings,
                &PixmapSettings {
                    x_scale: scale,
                    y_scale: scale,
                    bg_color: match options.background {
                        PdfBackground::White => WHITE,
                        PdfBackground::Transparent => TRANSPARENT,
                    },
                },
            )
        }))
        .map_err(|_| anyhow!("Page {page_number} could not be rendered."))?;
        control.check()?;
        let (width, height) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
        ensure!(
            (width, height) == expected[index],
            "Page {page_number} did not render at the requested dimensions."
        );
        Document::validate_size(width, height)
            .with_context(|| format!("Page {page_number} at {} DPI is too large", options.dpi))?;
        let bytes = pixmap.take_rgba8(ImageAlphaType::Alpha);
        let mut image = RgbaImage::from_raw(width, height, bytes)
            .with_context(|| format!("Page {page_number} could not be rendered."))?;
        options.background.flatten_background(&mut image);
        total_bytes = total_bytes.saturating_add(u64::from(width) * u64::from(height) * 4);
        ensure!(
            total_bytes <= MAX_STACK_BYTES,
            "The selected pages need too much memory. Use a lower resolution or fewer pages."
        );
        emit(page_number, image)?;
        progress(PdfProgress::Rendering {
            page: page_number,
            done: index + 1,
            total: expected.len(),
        });
    }
    control.check()?;
    Ok(())
}
