use crate::{Color, Padding};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TabBar {
    pub background: Color,
    pub spacing: f32,
    pub height: f32,
    pub padding: Padding,
}

impl TabBar {
    pub const FALLBACK: TabBar = TabBar {
        background: Color::new(0.9, 0.9, 0.9, 1.0),
        padding: Padding::new(0.0),
        spacing: 8.0,
        height: 50.0,
    };
}
