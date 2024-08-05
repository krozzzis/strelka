use std::path::Path;

use crate::theming::styles::button::Button;

use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub info: Info,
    pub button: Button,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub description: String,
}

pub async fn from_file(path: &Path) -> Result<Theme, String> {
    let text = tokio::fs::read_to_string(path).map_err(|e| e.to_string()).await?;
    let theme = toml::from_str(&text).map_err(|e| e.to_string())?;
    Ok(theme)
}
