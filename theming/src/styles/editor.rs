use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Editor {
    pub background: Color,
    pub text: Color,
    pub selection: Color,
    pub cursor: Color,
}

impl Editor {
    pub const FALLBACK: Editor = Editor {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        text: Color::new(0.0, 0.0, 0.0, 1.0),
        selection: Color::new(0.8, 0.4, 0.4, 1.0),
        cursor: Color::new(0.2, 0.8, 0.2, 1.0),
    };
}
