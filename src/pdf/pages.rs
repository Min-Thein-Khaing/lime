use anyhow::{Result, ensure};

use super::MAX_PDF_PAGES;
use crate::document::MAX_LAYERS;

pub const MAX_PAGE_SELECTION_CHARS: usize = 2048;

/// One-based pages and inclusive ranges, deduplicated in document order.
/// Bound endpoints before expanding them, including adversarial pasted input.
pub fn parse_page_selection(text: &str, page_count: usize) -> Result<Vec<usize>> {
    ensure!(
        (1..=MAX_PDF_PAGES).contains(&page_count),
        "The PDF page information is invalid."
    );
    ensure!(
        text.chars().count() <= MAX_PAGE_SELECTION_CHARS,
        "The page selection is too long."
    );
    ensure!(!text.trim().is_empty(), "Enter pages such as 1, 3-5, 8.");
    let normalized = text.replace(['–', '—'], "-");
    let mut selected = vec![false; page_count + 1];
    let number = |text: &str| -> Result<usize> {
        let text = text.trim();
        ensure!(
            !text.is_empty() && text.bytes().all(|ch| ch.is_ascii_digit()),
            "Enter pages such as 1, 3-5, 8."
        );
        let page = text
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Choose pages between 1 and {page_count}."))?;
        ensure!(
            (1..=page_count).contains(&page),
            "Choose pages between 1 and {page_count}."
        );
        Ok(page)
    };
    for part in normalized.split(',') {
        let mut range = part.split('-');
        let first = number(range.next().unwrap_or_default())?;
        let last = match range.next() {
            Some(last) => number(last)?,
            None => first,
        };
        ensure!(range.next().is_none(), "Enter pages such as 1, 3-5, 8.");
        ensure!(
            first <= last,
            "Page ranges must run from a lower page to a higher page."
        );
        selected[first..=last].fill(true);
    }
    Ok((1..=page_count).filter(|&page| selected[page]).collect())
}

pub fn validate_page_numbers(pages: &[usize], page_count: usize) -> Result<()> {
    ensure!(!pages.is_empty(), "Choose at least one PDF page.");
    ensure!(
        pages.len() <= MAX_LAYERS,
        "Choose at most {MAX_LAYERS} pages at a time."
    );
    ensure!(
        (1..=MAX_PDF_PAGES).contains(&page_count)
            && pages.iter().all(|&page| (1..=page_count).contains(&page)),
        "Choose pages between 1 and {page_count}."
    );
    ensure!(
        pages.windows(2).all(|pair| pair[0] < pair[1]),
        "Select each PDF page once, in document order."
    );
    Ok(())
}

/// Toggle a thumbnail while preserving every other page choice. Compact ranges
/// keep even a large source's All Pages selection within the text-field budget.
pub fn toggle_page_selection(
    text: &str,
    all_pages: bool,
    page: usize,
    page_count: usize,
) -> Result<String> {
    ensure!(
        (1..=MAX_PDF_PAGES).contains(&page_count) && (1..=page_count).contains(&page),
        "The PDF page is no longer available."
    );
    let mut pages = if all_pages {
        (1..=page_count).collect::<Vec<_>>()
    } else if text.trim().is_empty() {
        Vec::new()
    } else {
        parse_page_selection(text, page_count)?
    };
    match pages.binary_search(&page) {
        Ok(index) => {
            pages.remove(index);
        }
        Err(index) => pages.insert(index, page),
    }
    let mut ranges = Vec::new();
    let mut index = 0;
    while index < pages.len() {
        let first = pages[index];
        let mut last = first;
        index += 1;
        while index < pages.len() && pages[index] == last + 1 {
            last = pages[index];
            index += 1;
        }
        ranges.push(if first == last {
            first.to_string()
        } else {
            format!("{first}-{last}")
        });
    }
    let result = ranges.join(", ");
    ensure!(
        result.chars().count() <= MAX_PAGE_SELECTION_CHARS,
        "The page selection is too long. Use fewer ranges."
    );
    Ok(result)
}
