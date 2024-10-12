use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[cfg(feature = "async")]
use futures_core::Stream;
// #[cfg(feature = "iced")]
// use iced_futures::TryFutureExt;

use crate::{metadata::ThemeMetadata, Theme};

pub type ThemeID = core::smol_str::SmolStr;

pub enum CatalogError {
    IDNotFound,
    CannotReadFile,
    CannotParseThemeFile,
}

struct ThemeEntry<'a> {
    pub theme: Theme,
    pub metadata: ThemeMetadata<'a>,
}

/// Theme catalog.
///
/// Contains `ThemeId`'s and corresponding `Theme` and `ThemeMetadata`
#[derive(Default)]
pub struct Catalog {
    themes: HashMap<ThemeID, ThemeEntry<'static>>,
}

impl Catalog {
    /// Create an empty theme catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds new `Theme` and `ThemeMetadata` if that `ThemeId` did't added yet,
    /// otherwise replace with new one.
    pub fn insert(
        &mut self,
        id: impl Into<ThemeID>,
        theme: Theme,
        metadata: ThemeMetadata<'static>,
    ) {
        self.themes
            .insert(id.into(), ThemeEntry { theme, metadata });
    }

    /// Removes an `ThemeMetadata` by given `ThemeId` if one was added
    pub fn remove(&mut self, id: impl Into<ThemeID>) {
        self.themes.remove(&id.into());
    }

    /// List all `ThemeId` and corresponding `ThemeMetadata`
    pub fn list_metadata(&self) -> impl Iterator<Item = (&ThemeID, &ThemeMetadata<'static>)> {
        self.themes
            .iter()
            .map(|(id, ThemeEntry { theme: _, metadata })| (id, metadata))
    }

    /// List all `ThemeId` and corresponding `Theme`
    pub fn list_theme(&self) -> impl Iterator<Item = (&ThemeID, &Theme)> {
        self.themes
            .iter()
            .map(|(id, ThemeEntry { theme, metadata: _ })| (id, theme))
    }

    pub fn get_path(&self, id: &ThemeID) -> Option<PathBuf> {
        if let Some(entry) = self.themes.get(id) {
            entry.metadata.path.clone().map(|x| x.to_path_buf())
        } else {
            None
        }
    }

    pub fn get_theme(&self, id: impl Into<ThemeID>) -> Option<&Theme> {
        self.themes.get(&id.into()).map(|entry| &entry.theme)
    }

    pub fn get_metadata(&self, id: impl Into<ThemeID>) -> Option<&ThemeMetadata> {
        self.themes.get(&id.into()).map(|entry| &entry.metadata)
    }
}

pub async fn load_theme(path: impl AsRef<Path>) -> Result<(Theme, ThemeMetadata<'static>), String> {
    let path: &Path = path.as_ref();
    if path.is_dir() {
        let metadata_path = {
            let mut path = path.to_owned();
            path.push("metadata.toml");
            path
        };
        let theme_path = {
            let mut path = path.to_owned();
            path.push("theme.toml");
            path
        };
        if let Ok(metadata) = ThemeMetadata::from_file(&metadata_path).await {
            if let Ok(theme) = Theme::from_file(&theme_path).await {
                return Ok((theme, metadata));
            }
        }
    }
    Err(String::new())
}

pub async fn get_themes(
    dir: impl AsRef<Path>,
) -> impl Stream<Item = (Theme, ThemeMetadata<'static>)> {
    let mut dir_entries = tokio::fs::read_dir(dir).await.unwrap();

    async_stream::stream! {
        while let Some(entry) = dir_entries.next_entry().await.unwrap() {
            let path = entry.path();
            if path.is_dir() {
                let metadata_path = {
                    let mut path = path.clone();
                    path.push("metadata.toml");
                    path
                };
                let theme_path = {
                    let mut path = path.clone();
                    path.push("theme.toml");
                    path
                };
                if let Ok(metadata) = ThemeMetadata::from_file(&metadata_path).await {
                    if let Ok(theme) = Theme::from_file(&theme_path).await {
                        yield (theme, metadata)
                    }
                }
            }
        }
    }
}
