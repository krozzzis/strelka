use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContextMenu {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub width: f32,
    pub padding: f32,
    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
}

impl ContextMenu {
    pub const FALLBACK: ContextMenu = ContextMenu {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        border_color: Color::new(0.9, 0.9, 0.9, 1.0),
        border_width: 1.0,
        radius: 4.0,
        padding: 2.0,
        width: 200.0,
        shadow_x: 0.0,
        shadow_y: 0.0,
        shadow_blur: 0.0,
    };
}
