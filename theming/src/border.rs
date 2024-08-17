use serde::{Deserialize, Serialize};

use crate::Color;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Radius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Radius {
    pub const ZERO: Radius = Radius::new(0.0);

    pub const fn new(value: f32) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_left: value,
            bottom_right: value,
        }
    }
}

impl From<[f32; 4]> for Radius {
    fn from(value: [f32; 4]) -> Self {
        Self {
            top_left: value[0],
            top_right: value[1],
            bottom_left: value[2],
            bottom_right: value[3],
        }
    }
}

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_left: value,
            bottom_right: value,
        }
    }
}

#[cfg(feature = "iced")]
impl From<Radius> for iced_core::border::Radius {
    fn from(value: Radius) -> Self {
        Self {
            top_left: value.top_left,
            top_right: value.top_right,
            bottom_left: value.bottom_left,
            bottom_right: value.bottom_right,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Border {
    pub radius: Radius,
    pub color: Color,
    pub width: f32,
}

impl Border {
    pub const fn with_radius(radius: f32) -> Border {
        Border {
            radius: Radius::new(radius),
            color: Color::TRANSPARENT,
            width: 0.0,
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            radius: Radius::default(),
            color: Color::TRANSPARENT,
            width: 0.0,
        }
    }
}
