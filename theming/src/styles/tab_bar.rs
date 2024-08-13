use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TabBar {
    pub background: Color,
    pub padding: f32,
    pub spacing: f32,
}

impl TabBar {
    pub const FALLBACK: TabBar = TabBar {
        background: Color::new(0.9, 0.9, 0.9, 1.0),
        padding: 4.0,
        spacing: 4.0,
    };
}
