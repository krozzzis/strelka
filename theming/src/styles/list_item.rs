use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub hover: ListItemStyle,
    pub active: ListItemStyle,
    pub selected: ListItemStyle,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListItemStyle {
    pub text: Color,
    pub background: Color,
    pub radius: f32,
}

impl ListItem {
    pub const FALLBACK: ListItem = Self {
        hover: ListItemStyle {
            text: Color::BLACK,
            background: Color::new(0.8, 0.8, 0.8, 1.0),
            radius: 4.0,
        },
        active: ListItemStyle {
            text: Color::BLACK,
            background: Color::new(1.0, 1.0, 1.0, 1.0),
            radius: 4.0,
        },
        selected: ListItemStyle {
            text: Color::BLACK,
            background: Color::new(0.9, 0.9, 0.9, 1.0),
            radius: 4.0,
        },
    };
}
