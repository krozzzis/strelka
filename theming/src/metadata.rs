use std::{borrow::Cow, path::Path};

// #[cfg(feature = "iced")]
// use iced_futures::TryFutureExt;
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeMetadata<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub path: Option<Cow<'a, Path>>,
}

impl<'a> ThemeMetadata<'a> {
    #[cfg(feature = "load")]
    pub async fn from_file(path: &Path) -> Result<Self, ()> {
        let file_content = tokio::fs::read_to_string(path).await.map_err(|_| ())?;
        let metadata = toml::from_str(&file_content).map_err(|_| ())?;
        Ok(metadata)
    }
}

impl<'a> Default for ThemeMetadata<'a> {
    fn default() -> Self {
        Self {
            id: Cow::Borrowed("core.default"),
            name: Cow::Borrowed("Default"),
            path: None,
        }
    }
}
