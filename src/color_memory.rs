//! Personal RGB swatches and shared recent colors, separate from image history.
use anyhow::{Context as _, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub const DEFAULT_SWATCHES: [u32; 2] = [0x000000, 0xffffff];
pub const RECENT_LIMIT: usize = 12;
const MAX_BYTES: u64 = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Memory {
    pub swatches: [u32; 2],
    pub recent: Vec<u32>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            swatches: DEFAULT_SWATCHES,
            recent: Vec::new(),
        }
    }
}

/// Bounded edits can be replayed over a late startup load or another writer's
/// saved colors without replacing untouched swatches or losing recent choices.
#[derive(Clone, Debug, Default)]
pub struct Changes {
    swatches: [Option<u32>; 2],
    recent: Vec<u32>,
}

fn remember(recent: &mut Vec<u32>, color: u32) {
    recent.retain(|&value| value != color);
    recent.insert(0, color);
    recent.truncate(RECENT_LIMIT);
}

impl Changes {
    pub fn color(color: u32) -> Self {
        Self {
            recent: vec![color & 0xffffff],
            ..Self::default()
        }
    }

    pub fn swatch(index: usize, color: u32) -> Self {
        let mut changes = Self::color(color);
        changes.swatches[index] = Some(color & 0xffffff);
        changes
    }

    pub fn swatches(swatches: [u32; 2]) -> Self {
        Self {
            swatches: swatches.map(|color| Some(color & 0xffffff)),
            ..Self::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.swatches.iter().all(Option::is_none) && self.recent.is_empty()
    }

    pub fn merge(&mut self, newer: &Self) {
        for (current, new) in self.swatches.iter_mut().zip(newer.swatches) {
            if new.is_some() {
                *current = new;
            }
        }
        for &color in newer.recent.iter().rev() {
            remember(&mut self.recent, color);
        }
    }
}

impl Memory {
    pub fn valid(&self) -> bool {
        self.recent.len() <= RECENT_LIMIT
            && self
                .swatches
                .iter()
                .chain(&self.recent)
                .all(|&color| color <= 0xffffff)
            && self
                .recent
                .iter()
                .enumerate()
                .all(|(index, color)| !self.recent[..index].contains(color))
    }

    pub fn apply(&mut self, changes: &Changes) {
        for (swatch, color) in self.swatches.iter_mut().zip(changes.swatches) {
            if let Some(color) = color {
                *swatch = color;
            }
        }
        for &color in changes.recent.iter().rev() {
            remember(&mut self.recent, color);
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireMemory {
    version: u32,
    swatches: [u32; 2],
    recent: Vec<u32>,
}

fn decode(bytes: &[u8]) -> Result<Memory> {
    ensure!(
        bytes.len() as u64 <= MAX_BYTES,
        "Color memory is too large."
    );
    let wire: WireMemory = serde_json::from_slice(bytes).context("Invalid color memory")?;
    ensure!(wire.version == 1, "Unsupported color memory version.");
    ensure!(wire.recent.len() <= RECENT_LIMIT, "Too many recent colors.");
    ensure!(
        wire.swatches
            .iter()
            .chain(&wire.recent)
            .all(|&color| color <= 0xffffff),
        "Invalid RGB color."
    );
    let mut recent = Vec::new();
    for color in wire.recent.into_iter().rev() {
        remember(&mut recent, color);
    }
    Ok(Memory {
        swatches: wire.swatches,
        recent,
    })
}

pub fn default_path() -> Option<PathBuf> {
    crate::gradient::library::default_path().map(|path| path.with_file_name("colors.json"))
}

pub fn load(path: &Path) -> Result<Memory> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Memory::default()),
        Err(error) => return Err(error).context("Could not open color memory"),
    };
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
    decode(&bytes)
}
