use crate::theming::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TabBar {
    pub background: Color,
    pub padding: f32,
    pub spacing: f32,
}
