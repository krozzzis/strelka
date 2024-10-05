use std::path::PathBuf;

use config::Config;
use core::{document::DocumentStore, pane::PaneModel, value::Value};
use iced::{advanced::graphics::core::SmolStr, widget::text_editor::Content};
use theming::{
    catalog::{Catalog, ThemeID},
    Theme,
};

pub struct State {
    pub documents: DocumentStore<Content>,
    pub panes: PaneModel,
    pub themes: Catalog,
    pub config: Config,
    pub working_directory: PathBuf,
}

impl State {
    pub fn get_theme(&self) -> Theme {
        let id = if let Some(Value::String(str)) = self.config.get("core", "theme") {
            SmolStr::new(str)
        } else {
            SmolStr::new("core.light")
        };

        if let Some(theme) = self.themes.get_theme(id) {
            theme.clone()
        } else {
            Theme::default()
        }
    }

    pub fn set_theme(&mut self, id: ThemeID) {
        self.config
            .insert("core", "theme", Value::String(id.to_string()));
        if let Ok(mut theme) = theming::THEME.write() {
            *theme = self.get_theme();
        }
    }
}
