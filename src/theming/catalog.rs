use std::{collections::HashMap, path::PathBuf};

use iced::{advanced::graphics::core::SmolStr, futures::TryFutureExt};

use crate::theming::{metadata::ThemeMetadata, Theme};

pub type ThemeID = SmolStr;

pub enum CatalogError {
    IDNotFound,
    CannotReadFile,
    CannotParseThemeFile,
}

/// Theme catalog.
///
/// Contains `ThemeId`'s and corresponding `ThemeMetadata`
pub struct Catalog<'a> {
    pub themes: HashMap<ThemeID, ThemeMetadata<'a>>,
}

impl<'a> Catalog<'a> {
    /// Create an empty theme catalog.
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
        }
    }

    /// Adds new `ThemeMetadata` if that `ThemeId` did't added yet,
    /// otherwise replace with new one.
    pub fn insert(&mut self, id: impl Into<ThemeID>, meta: ThemeMetadata<'a>) {
        self.themes.insert(id.into(), meta);
    }

    /// Removes an `ThemeMetadata` by given `ThemeId` if one was added
    pub fn remove(&mut self, id: impl Into<ThemeID>) {
        self.themes.remove(&id.into());
    }

    /// List all `ThemeId` and corresponding `ThemeMetadata`
    pub fn themes(&self) -> impl Iterator<Item = (&ThemeID, &ThemeMetadata<'a>)> {
        self.themes.iter()
    }

    pub fn get_path(&self, id: &ThemeID) -> Option<PathBuf> {
        self.themes.get(id).map(|x| x.path.to_path_buf())
    }

    pub async fn load(&self, id: &ThemeID) -> Result<Theme<'_>, CatalogError> {
        if let Some(meta) = self.themes.get(id) {
            let path = meta.path.as_ref();
            let text = tokio::fs::read_to_string(path)
                .map_err(|_| CatalogError::CannotReadFile)
                .await?;
            let theme: Theme =
                toml::from_str(&text).map_err(|_| CatalogError::CannotParseThemeFile)?;
            Ok(theme)
        } else {
            Err(CatalogError::IDNotFound)
        }
    }
}
