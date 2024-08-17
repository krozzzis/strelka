use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Margin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Margin {
    pub const ZERO: Margin = Margin::new(0.0);

    pub const fn new(value: f32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl From<[f32; 4]> for Margin {
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
impl From<Margin> for iced_core::Padding {
    fn from(value: Margin) -> Self {
        Self {
            top: value.top,
            bottom: value.bottom,
            left: value.left,
            right: value.right,
        }
    }
}
