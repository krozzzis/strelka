use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileExplorer {
    pub background: Color,
    pub text: Color,
    pub padding: f32,
    pub width: f32,
}

impl FileExplorer {
    pub const FALLBACK: FileExplorer = FileExplorer {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        text: Color::new(0.0, 0.0, 0.0, 1.0),
        padding: 4.0,
        width: 300.0,
    };
}
