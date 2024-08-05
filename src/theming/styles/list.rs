use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct List {
    pub padding: f32,
    pub spacing: f32,
}

impl List {
    pub const FALLBACK: List = List {
        padding: 8.0,
        spacing: 4.0,
    };
}
