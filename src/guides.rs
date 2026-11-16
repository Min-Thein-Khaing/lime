use serde::{Deserialize, Serialize};

pub const MAX_GUIDES: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Guide {
    pub axis: Axis,
    pub position: u32,
}

impl Guide {
    pub fn valid(self, size: (u32, u32)) -> bool {
        self.position
            <= match self.axis {
                Axis::Horizontal => size.1,
                Axis::Vertical => size.0,
            }
    }

    pub fn translated(self, delta: (i64, i64), size: (u32, u32)) -> Option<Self> {
        let position = i64::from(self.position)
            + match self.axis {
                Axis::Horizontal => delta.1,
                Axis::Vertical => delta.0,
            };
        let guide = Self {
            axis: self.axis,
            position: u32::try_from(position).ok()?,
        };
        guide.valid(size).then_some(guide)
    }

    pub fn scaled(self, old: (u32, u32), new: (u32, u32)) -> Self {
        let (before, after) = match self.axis {
            Axis::Horizontal => (old.1, new.1),
            Axis::Vertical => (old.0, new.0),
        };
        Self {
            position: ((u64::from(self.position) * u64::from(after) + u64::from(before) / 2)
                / u64::from(before)) as u32,
            ..self
        }
    }

    pub fn turned(self, size: (u32, u32), turn: crate::transform::Turn) -> Self {
        use crate::transform::Turn;
        match (self.axis, turn) {
            (Axis::Vertical, Turn::Right) => Self {
                axis: Axis::Horizontal,
                position: self.position,
            },
            (Axis::Horizontal, Turn::Right) => Self {
                axis: Axis::Vertical,
                position: size.1 - self.position,
            },
            (Axis::Vertical, Turn::Left) => Self {
                axis: Axis::Horizontal,
                position: size.0 - self.position,
            },
            (Axis::Horizontal, Turn::Left) => Self {
                axis: Axis::Vertical,
                position: self.position,
            },
            (Axis::Vertical, Turn::Half) => Self {
                position: size.0 - self.position,
                ..self
            },
            (Axis::Horizontal, Turn::Half) => Self {
                position: size.1 - self.position,
                ..self
            },
        }
    }

    pub fn mirrored(self, size: (u32, u32), mirror: crate::transform::Mirror) -> Self {
        match (self.axis, mirror) {
            (Axis::Vertical, crate::transform::Mirror::Horizontal) => Self {
                position: size.0 - self.position,
                ..self
            },
            (Axis::Horizontal, crate::transform::Mirror::Vertical) => Self {
                position: size.1 - self.position,
                ..self
            },
            _ => self,
        }
    }
}

// Snap pointer moves only. Explicit numeric positions and keyboard steps stay exact.
pub fn snap_move(
    offset: (i32, i32),
    size: (u32, u32),
    canvas: (u32, u32),
    grid: Option<u32>,
    guides: &[Guide],
    tolerance: f64,
) -> (i32, i32) {
    let axis = |origin: i32, extent: u32, limit: u32, axis: Axis| {
        let mut best: Option<(f64, usize, i32)> = None;
        for (anchor_index, anchor) in [0., f64::from(extent), f64::from(extent) / 2.]
            .into_iter()
            .enumerate()
        {
            let mut candidate = |target: u32, priority: usize| {
                let position = (f64::from(target) - anchor).round();
                if position < f64::from(i32::MIN) || position > f64::from(i32::MAX) {
                    return;
                }
                let distance = (position - f64::from(origin)).abs();
                let rank = (distance, priority * 3 + anchor_index, position as i32);
                if distance <= tolerance && best.is_none_or(|best| rank < best) {
                    best = Some(rank);
                }
            };
            for guide in guides
                .iter()
                .filter(|guide| guide.axis == axis && guide.position <= limit)
            {
                candidate(guide.position, 0);
            }
            if let Some(spacing) = grid.filter(|&spacing| spacing > 0) {
                let target = ((f64::from(origin) + anchor) / f64::from(spacing)).round()
                    * f64::from(spacing);
                if (0. ..=f64::from(limit)).contains(&target) {
                    candidate(target as u32, 1);
                }
            }
        }
        best.map_or(origin, |best| best.2)
    };
    if !tolerance.is_finite() || tolerance < 0. {
        return offset;
    }
    (
        axis(offset.0, size.0, canvas.0, Axis::Vertical),
        axis(offset.1, size.1, canvas.1, Axis::Horizontal),
    )
}

// Thin the visible dots at low zoom, without changing the image-space snap grid.
pub fn display_spacing(spacing: u32, pixels_per_unit: f32, minimum: f32) -> f32 {
    let mut step = spacing.max(1) as f32;
    if !pixels_per_unit.is_finite() || pixels_per_unit <= 0. {
        return step;
    }
    while step * pixels_per_unit < minimum {
        step *= 2.;
    }
    step
}
