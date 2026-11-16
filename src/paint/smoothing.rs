use std::time::Duration;

pub const TICK: Duration = Duration::from_millis(8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Smoothing {
    #[default]
    Off,
    Light,
    Strong,
}

impl Smoothing {
    fn response(self) -> Duration {
        Duration::from_millis(match self {
            Self::Off => 0,
            Self::Light => 16,
            Self::Strong => 32,
        })
    }

    fn settle(self) -> Duration {
        self.response() * 4
    }
}

/// A causal, time-based low pass with a finite settling deadline. Coordinates
/// are layer-local; timing is relative to the beginning of one stroke.
/// No input queue grows with pointer frequency or the duration of a gesture.
pub struct StrokeSmoother {
    mode: Smoothing,
    position: (f64, f64),
    target: (f64, f64),
    updated: Duration,
    input_at: Duration,
}

impl StrokeSmoother {
    pub fn new(mode: Smoothing, point: (f64, f64)) -> Option<Self> {
        if !point.0.is_finite() || !point.1.is_finite() {
            return None;
        }
        Some(Self {
            mode,
            position: point,
            target: point,
            updated: Duration::ZERO,
            input_at: Duration::ZERO,
        })
    }

    pub fn pending(&self) -> bool {
        self.position != self.target
    }

    pub fn input(&mut self, point: (f64, f64), at: Duration) -> Vec<(f64, f64)> {
        if !point.0.is_finite() || !point.1.is_finite() || at < self.updated {
            return Vec::new();
        }
        let mut points = self.advance(at);
        if self.target != point {
            self.target = point;
            self.input_at = at;
            if self.mode == Smoothing::Off {
                self.position = point;
                points.push(point);
            }
        }
        points
    }

    pub fn advance(&mut self, at: Duration) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        while self.updated < at && self.pending() {
            let next = self.updated.saturating_add(TICK).min(at);
            if self.mode == Smoothing::Off
                || next == Duration::MAX
                || next.saturating_sub(self.input_at) >= self.mode.settle()
            {
                self.position = self.target;
            } else {
                let remaining = (-(next - self.updated).as_secs_f64()
                    / self.mode.response().as_secs_f64())
                .exp();
                // A convex blend avoids overflowing a difference between large
                // finite coordinates and never overshoots either endpoint.
                self.position = (
                    self.position.0 * remaining + self.target.0 * (1. - remaining),
                    self.position.1 * remaining + self.target.1 * (1. - remaining),
                );
            }
            self.updated = next;
            points.push(self.position);
        }
        self.updated = self.updated.max(at);
        points
    }

    /// Drain the small remaining tail synchronously on release. This uses the
    /// same curve as the live timer, reaches the exact endpoint, and adds no
    /// asynchronous continuation to a committed stroke.
    pub fn finish(&mut self) -> Vec<(f64, f64)> {
        let mut points = self.advance(self.updated.saturating_add(self.mode.settle()));
        if self.pending() {
            self.position = self.target;
            points.push(self.position);
        }
        points
    }
}
