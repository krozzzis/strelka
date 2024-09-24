use std::path::PathBuf;

use core::{document::DocumentStore, pane::PaneModel};
use iced::widget::text_editor::Content;
use theming::{
    catalog::{Catalog, ThemeID},
    Theme,
};

pub struct State {
    pub documents: DocumentStore<Content>,
    pub panes: PaneModel,
    pub theme: ThemeID,
    pub themes: Catalog,
    pub working_directory: PathBuf,
}

impl State {
    pub fn get_theme(&self) -> Theme {
        let id = self.theme.clone();
        if let Some(theme) = self.themes.get_theme(id) {
            theme.clone()
        } else {
            Theme::default()
        }
    }

    pub fn set_theme(&mut self, id: ThemeID) {
        self.theme = id;
        if let Ok(mut theme) = theming::THEME.write() {
            *theme = self.get_theme();
        }
    }
}
