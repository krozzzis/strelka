use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub hover: ButtonStyle,
    pub active: ButtonStyle,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    pub text: Color,
    pub background: Color,
    pub radius: f32,
}

impl Button {
    pub const FALLBACK: Button = Button {
        hover: ButtonStyle {
            text: Color::BLACK,
            background: Color::new(0.8, 0.8, 0.8, 1.0),
            radius: 4.0,
        },
        active: ButtonStyle {
            text: Color::BLACK,
            background: Color::new(0.9, 0.9, 0.9, 1.0),
            radius: 4.0,
        },
    };
}
