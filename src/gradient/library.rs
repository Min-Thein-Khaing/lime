use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result, bail, ensure};
use image::Rgba;
use serde::{Deserialize, Serialize};

use super::{ColorRamp, ColorStop};
use crate::paint::GradientShape;

pub const MAX_PRESETS: usize = 32;
pub const MAX_NAME_CHARS: usize = 64;
const MAX_BYTES: u64 = 256 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresetValue {
    pub ramp: ColorRamp,
    pub shape: GradientShape,
    pub reverse: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preset {
    pub id: u64,
    pub name: String,
    pub value: PresetValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Library {
    next_id: u64,
    presets: Vec<Preset>,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            next_id: 1,
            presets: Vec::new(),
        }
    }
}

impl Library {
    pub fn presets(&self) -> &[Preset] {
        &self.presets
    }

    pub fn get(&self, id: u64) -> Option<&Preset> {
        self.presets.iter().find(|preset| preset.id == id)
    }

    pub fn suggested_name(&self) -> String {
        (1..=MAX_PRESETS + 1)
            .map(|index| format!("Gradient {index}"))
            .find(|name| {
                !self
                    .presets
                    .iter()
                    .any(|preset| preset.name.eq_ignore_ascii_case(name))
            })
            .unwrap_or_else(|| "Gradient".into())
    }

    fn name(&self, text: &str, except: Option<u64>) -> Result<String> {
        let name = checked_name(text)?;
        let key = name.to_lowercase();
        ensure!(
            !self
                .presets
                .iter()
                .any(|preset| Some(preset.id) != except && preset.name.to_lowercase() == key),
            "A gradient with that name already exists."
        );
        Ok(name)
    }

    pub fn insert(&mut self, name: &str, value: PresetValue) -> Result<u64> {
        ensure!(
            self.presets.len() < MAX_PRESETS,
            "The gradient library can hold at most {MAX_PRESETS} gradients."
        );
        let name = self.name(name, None)?;
        let next = self
            .next_id
            .checked_add(1)
            .context("Gradient identifiers are exhausted.")?;
        let id = self.next_id;
        self.presets.push(Preset { id, name, value });
        self.next_id = next;
        Ok(id)
    }

    pub fn rename(&mut self, id: u64, name: &str) -> Result<bool> {
        let name = self.name(name, Some(id))?;
        let preset = self
            .presets
            .iter_mut()
            .find(|preset| preset.id == id)
            .context("That gradient is no longer available.")?;
        if preset.name == name {
            return Ok(false);
        }
        preset.name = name;
        Ok(true)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.presets.len();
        self.presets.retain(|preset| preset.id != id);
        self.presets.len() != before
    }
}

fn checked_name(text: &str) -> Result<String> {
    let name = text.trim();
    ensure!(!name.is_empty(), "Enter a gradient name.");
    ensure!(
        name.chars().count() <= MAX_NAME_CHARS && !name.chars().any(char::is_control),
        "Use at most {MAX_NAME_CHARS} characters without control characters."
    );
    Ok(name.to_owned())
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireLibrary {
    version: u32,
    next_id: u64,
    presets: Vec<WirePreset>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WirePreset {
    id: u64,
    name: String,
    stops: Vec<WireStop>,
    shape: WireShape,
    reverse: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireStop {
    position: u16,
    color: [u8; 4],
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WireShape {
    Linear,
    Radial,
}

pub fn decode(bytes: &[u8]) -> Result<Library> {
    ensure!(
        bytes.len() as u64 <= MAX_BYTES,
        "The gradient library is too large."
    );
    let wire: WireLibrary =
        serde_json::from_slice(bytes).context("The gradient library is invalid")?;
    ensure!(
        wire.version == 1,
        "This gradient library version is not supported."
    );
    ensure!(
        wire.presets.len() <= MAX_PRESETS && wire.next_id > 0,
        "Invalid gradient library limits."
    );
    let mut ids = HashSet::new();
    let mut names = HashSet::new();
    let mut presets = Vec::with_capacity(wire.presets.len());
    for preset in wire.presets {
        ensure!(
            preset.id > 0 && preset.id < wire.next_id && ids.insert(preset.id),
            "Invalid gradient identifiers."
        );
        let name = checked_name(&preset.name)?;
        ensure!(
            names.insert(name.to_lowercase()),
            "Duplicate gradient names."
        );
        let stops: Vec<_> = preset
            .stops
            .into_iter()
            .map(|stop| ColorStop {
                position: stop.position,
                color: Rgba(stop.color),
            })
            .collect();
        let ramp = ColorRamp::from_stops(&stops).context("Invalid gradient color stops.")?;
        presets.push(Preset {
            id: preset.id,
            name,
            value: PresetValue {
                ramp,
                shape: match preset.shape {
                    WireShape::Linear => GradientShape::Linear,
                    WireShape::Radial => GradientShape::Radial,
                },
                reverse: preset.reverse,
            },
        });
    }
    Ok(Library {
        next_id: wire.next_id,
        presets,
    })
}

pub fn encode(library: &Library) -> Result<Vec<u8>> {
    let wire = WireLibrary {
        version: 1,
        next_id: library.next_id,
        presets: library
            .presets
            .iter()
            .map(|preset| WirePreset {
                id: preset.id,
                name: preset.name.clone(),
                stops: preset
                    .value
                    .ramp
                    .stops()
                    .iter()
                    .map(|stop| WireStop {
                        position: stop.position,
                        color: stop.color.0,
                    })
                    .collect(),
                shape: match preset.value.shape {
                    GradientShape::Linear => WireShape::Linear,
                    GradientShape::Radial => WireShape::Radial,
                },
                reverse: preset.value.reverse,
            })
            .collect(),
    };
    let bytes = serde_json::to_vec_pretty(&wire)?;
    // Keep the same validation contract in both directions.
    decode(&bytes)?;
    Ok(bytes)
}

pub fn default_path() -> Option<PathBuf> {
    let path = if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|root| {
            PathBuf::from(root).join("Library/Application Support/com.eimg.lime/gradients.json")
        })
    } else if cfg!(target_os = "windows") {
        std::env::var_os("LOCALAPPDATA").map(|root| PathBuf::from(root).join("Lime/gradients.json"))
    } else {
        xdg_library_path(
            std::env::var_os("XDG_CONFIG_HOME"),
            std::env::var_os("HOME"),
        )
    };
    path.filter(|path| path.is_absolute())
}

fn xdg_library_path(
    config: Option<std::ffi::OsString>,
    home: Option<std::ffi::OsString>,
) -> Option<PathBuf> {
    config
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home.map(PathBuf::from)
                .filter(|path| path.is_absolute())
                .map(|path| path.join(".config"))
        })
        .map(|root| root.join("lime/gradients.json"))
}

fn read_bytes(path: &Path) -> Result<Option<Vec<u8>>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error).context("The gradient library could not be opened"),
    };
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_BYTES,
        "The gradient library is too large."
    );
    Ok(Some(bytes))
}

pub fn load(path: &Path) -> Result<(Library, Option<Vec<u8>>)> {
    let bytes = read_bytes(path)?;
    let library = bytes
        .as_ref()
        .map_or_else(|| Ok(Library::default()), |bytes| decode(bytes))?;
    Ok((library, bytes))
}

pub fn save(path: &Path, expected: Option<&[u8]>, library: &Library) -> Result<Vec<u8>> {
    let bytes = encode(library)?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .context("No gradient library directory is available.")?;
    fs::create_dir_all(parent).context("The gradient library directory could not be created")?;
    // The OS releases this advisory lock after a crash, so no stale lock cleanup
    // is needed. It serializes Lime writers without blocking the UI thread.
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(parent.join("gradients.lock"))?;
    lock.try_lock()
        .context("Another Lime window is saving gradients. Reload and try again")?;
    let current = read_bytes(path)?;
    if current.as_deref() != expected {
        bail!("The gradient library changed outside this window. Reload it before saving.");
    }
    if let Some(bytes) = &current {
        decode(bytes).context("The existing gradient library is not writable by this version")?;
    }
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(&bytes)?;
    temporary.as_file().sync_all()?;
    if read_bytes(path)?.as_deref() != expected {
        bail!("The gradient library changed outside this window. Reload it before saving.");
    }
    temporary
        .persist(path)
        .context("The gradient library could not be replaced")?;
    Ok(bytes)
}
