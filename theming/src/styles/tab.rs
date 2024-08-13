use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub hover: TabStyle,
    pub active: TabStyle,
    pub selected: TabStyle,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TabStyle {
    pub text: Color,
    pub background: Color,
    pub radius: f32,
    pub height: f32,
}

impl Tab {
    pub const FALLBACK: Tab = Self {
        hover: TabStyle {
            text: Color::BLACK,
            background: Color::new(0.8, 0.8, 0.8, 1.0),
            radius: 4.0,
            height: 32.0,
        },
        active: TabStyle {
            text: Color::BLACK,
            background: Color::new(1.0, 1.0, 1.0, 1.0),
            radius: 4.0,
            height: 32.0,
        },
        selected: TabStyle {
            text: Color::BLACK,
            background: Color::new(0.9, 0.9, 0.9, 1.0),
            radius: 4.0,
            height: 32.0,
        },
    };
}
