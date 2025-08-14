use std::fs::read_to_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub scale_factor: f32,
    pub decorations: bool,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            decorations: true,
        }
    }
}

pub fn load_config() -> ApplicationConfig {
    if let Ok(config_text) = read_to_string("./strelka.ron") {
        ron::from_str(&config_text).unwrap_or_default()
    } else {
        ApplicationConfig::default()
    }
}
