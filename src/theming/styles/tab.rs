use crate::theming::color::Color;
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
}
