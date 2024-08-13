#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct List {
    pub spacing: f32,
}

impl List {
    pub const FALLBACK: List = List { spacing: 4.0 };
}
