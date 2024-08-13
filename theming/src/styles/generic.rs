use crate::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub background: Color,
    pub background2: Color,
    pub text: Color,
}

impl Generic {
    pub const FALLBACK: Generic = Generic {
        background: Color::new(1.0, 1.0, 1.0, 1.0),
        background2: Color::new(0.8, 0.8, 0.8, 1.0),
        text: Color::new(0.0, 0.0, 0.0, 1.0),
    };
}
