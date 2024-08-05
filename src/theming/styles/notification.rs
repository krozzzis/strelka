use crate::theming::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub background: Color,
    pub text: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
}

impl Notification {
    pub const FALLBACK: Notification = Notification {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        text: Color::new(0.0, 0.0, 0.0, 1.0),
        border_color: Color::new(0.9, 0.9, 0.9, 1.0),
        border_width: 1.0,
        radius: 4.0,
        shadow_x: 0.0,
        shadow_y: 0.0,
        shadow_blur: 0.0,
    };
}
