use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Padding {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for Padding {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Padding {
    pub const ZERO: Padding = Padding::new(0.0);

    pub const fn new(value: f32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl From<[f32; 4]> for Padding {
    fn from(value: [f32; 4]) -> Self {
        Self {
            left: value[0],
            top: value[1],
            right: value[2],
            bottom: value[3],
        }
    }
}

#[cfg(feature = "iced")]
impl From<Padding> for iced_core::Padding {
    fn from(value: Padding) -> Self {
        Self {
            top: value.top,
            bottom: value.bottom,
            left: value.left,
            right: value.right,
        }
    }
}
