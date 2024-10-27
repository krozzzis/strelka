use core::ThemeId;
use std::{collections::HashMap, sync::Arc};

use crate::{index::ThemeIndex, Theme};

/// Theme catalog.
///
/// Contains `ThemeId`'s and corresponding `Theme` and `ThemeMetadata`
#[derive(Default)]
pub struct Catalog {
    cache: HashMap<ThemeId, Arc<Theme>>,
    index: ThemeIndex,
    current_theme: Arc<Theme>,
}

impl Catalog {
    /// Create an empty theme catalog.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_current_theme(&self) -> Arc<Theme> {
        self.current_theme.clone()
    }

    pub fn set_index(&mut self, index: ThemeIndex) {
        self.index = index;
    }

    pub async fn load_theme(&mut self, id: ThemeId) -> Option<Arc<Theme>> {
        if let Some(theme) = self.cache.get(&id) {
            Some(theme.clone())
        } else if let Some(path) = self.index.get_path(&id) {
            if let Ok(theme) = Theme::from_file(path).await {
                let theme = Arc::new(theme);

                self.cache.insert(id, theme.clone());

                Some(theme)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn set_theme(&mut self, id: ThemeId) {
        if let Some(theme) = self.load_theme(id).await {
            self.current_theme = theme;
        }
    }
}
