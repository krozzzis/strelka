use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Editor {
    pub background: Color,
    pub background2: Color,
    pub text: Color,
    pub selection: Color,
    pub cursor: Color,
    pub radius: f32,
    pub padding: f32,
    pub width: f32,
}

impl Editor {
    pub const FALLBACK: Editor = Editor {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        background2: Color::new(1.0, 1.0, 1.0, 1.0),
        text: Color::new(0.0, 0.0, 0.0, 1.0),
        selection: Color::new(0.8, 0.4, 0.4, 1.0),
        cursor: Color::new(0.2, 0.8, 0.2, 1.0),
        radius: 0.0,
        padding: 0.0,
        width: 500.0,
    };
}
