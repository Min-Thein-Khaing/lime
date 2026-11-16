use image::Rgba;

pub mod library;

pub const MAX_STOPS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ColorStop {
    pub position: u16,
    #[serde(with = "crate::settings::rgba")]
    pub color: Rgba<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorRamp {
    stops: [ColorStop; MAX_STOPS],
    count: usize,
}

impl PartialEq for ColorRamp {
    fn eq(&self, other: &Self) -> bool {
        self.stops() == other.stops()
    }
}
impl Eq for ColorRamp {}

impl serde::Serialize for ColorRamp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(self.stops(), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ColorRamp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let stops = <Vec<ColorStop> as serde::Deserialize>::deserialize(deserializer)?;
        Self::from_stops(&stops).ok_or_else(|| serde::de::Error::custom("Invalid gradient stops"))
    }
}

impl ColorRamp {
    pub fn from_stops(stops: &[ColorStop]) -> Option<Self> {
        if !(2..=MAX_STOPS).contains(&stops.len())
            || stops.first()?.position != 0
            || stops.last()?.position != 1000
            || stops
                .windows(2)
                .any(|pair| pair[0].position >= pair[1].position)
        {
            return None;
        }
        let mut ramp = Self::new(stops[0].color, stops[stops.len() - 1].color);
        ramp.stops[..stops.len()].copy_from_slice(stops);
        ramp.count = stops.len();
        Some(ramp)
    }

    pub fn new(start: Rgba<u8>, end: Rgba<u8>) -> Self {
        let mut stops = [ColorStop {
            position: 0,
            color: start,
        }; MAX_STOPS];
        stops[1] = ColorStop {
            position: 1000,
            color: end,
        };
        Self { stops, count: 2 }
    }

    pub fn stops(&self) -> &[ColorStop] {
        &self.stops[..self.count]
    }

    pub fn sample(&self, t: f64) -> Rgba<u8> {
        let position = if t.is_finite() {
            t.clamp(0., 1.) * 1000.
        } else {
            0.
        };
        let pair = self
            .stops()
            .windows(2)
            .find(|pair| position <= f64::from(pair[1].position))
            .unwrap();
        let t = (position - f64::from(pair[0].position))
            / f64::from(pair[1].position - pair[0].position);
        interpolate(pair[0].color, pair[1].color, t)
    }

    pub fn add_stop(&mut self) -> Option<usize> {
        if self.count == MAX_STOPS {
            return None;
        }
        let gap = self
            .stops()
            .windows(2)
            .enumerate()
            .max_by_key(|(index, pair)| {
                (
                    pair[1].position - pair[0].position,
                    std::cmp::Reverse(*index),
                )
            })?
            .0;
        let position = (self.stops[gap].position + self.stops[gap + 1].position) / 2;
        if position <= self.stops[gap].position {
            return None;
        }
        let color = self.sample(f64::from(position) / 1000.);
        let index = gap + 1;
        self.stops.copy_within(index..self.count, index + 1);
        self.stops[index] = ColorStop { position, color };
        self.count += 1;
        Some(index)
    }

    pub fn move_stop(&mut self, index: usize, position: u16) -> bool {
        if index == 0 || index >= self.count - 1 {
            return false;
        }
        let position = position.clamp(
            self.stops[index - 1].position + 1,
            self.stops[index + 1].position - 1,
        );
        if position == self.stops[index].position {
            return false;
        }
        self.stops[index].position = position;
        true
    }

    pub fn remove_stop(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.count - 1 {
            return false;
        }
        self.stops.copy_within(index + 1..self.count, index);
        self.count -= 1;
        true
    }

    pub fn set_color(&mut self, index: usize, color: Rgba<u8>) -> bool {
        let Some(stop) = self.stops[..self.count].get_mut(index) else {
            return false;
        };
        if stop.color == color {
            return false;
        }
        stop.color = color;
        true
    }

    pub fn map_colors(mut self, map: impl Fn(Rgba<u8>) -> Rgba<u8>) -> Self {
        for stop in &mut self.stops[..self.count] {
            stop.color = map(stop.color);
        }
        self
    }
}

pub fn interpolate(start: Rgba<u8>, end: Rgba<u8>, t: f64) -> Rgba<u8> {
    let t = t.clamp(0., 1.);
    let alpha = f64::from(start[3]) * (1. - t) + f64::from(end[3]) * t;
    let a = alpha.round().clamp(0., 255.) as u8;
    if a == 0 {
        return Rgba([0; 4]);
    }
    let channel = |index| {
        ((f64::from(start[index]) * f64::from(start[3]) * (1. - t)
            + f64::from(end[index]) * f64::from(end[3]) * t)
            / alpha)
            .round()
            .clamp(0., 255.) as u8
    };
    Rgba([channel(0), channel(1), channel(2), a])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_stops_do_not_tint_visible_colors_or_darken_faint_alpha() {
        let ramp = ColorRamp::new(Rgba([255, 0, 0, 255]), Rgba([0, 0, 255, 0]));
        assert_eq!(ramp.sample(0.5), Rgba([255, 0, 0, 128]));
        assert_eq!(ramp.sample(1.), Rgba([0; 4]));
        assert_eq!(ramp.sample(-2.), Rgba([255, 0, 0, 255]));
        assert_eq!(ramp.sample(f64::NAN), ramp.sample(0.));
        let faint = ColorRamp::new(Rgba([20, 120, 240, 1]), Rgba([255, 0, 0, 0]));
        assert_eq!(faint.sample(0.25), Rgba([20, 120, 240, 1]));
        assert_eq!(faint.sample(0.75), Rgba([0; 4]));
    }
}
