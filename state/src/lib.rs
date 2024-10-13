use config::Config;
use core::{document::DocumentStore, pane::PaneModel, smol_str::SmolStr, value::Value, ThemeID};
use iced::widget::text_editor::Content;
use log::info;
use std::sync::Arc;
use theming::{catalog::Catalog, index::ThemeIndex, Theme};

pub struct State {
    pub documents: DocumentStore<Content>,
    pub panes: PaneModel,
    pub themes: Catalog,
    pub config: Config,
}

impl State {
    pub async fn make_theme_index(&mut self) {
        let index = ThemeIndex::load_from_directory("./themes/").await;
        if let Ok(index) = index {
            self.themes.set_index(index);
        }
    }

    pub fn get_theme(&self) -> Arc<Theme> {
        self.themes.get_current_theme()
    }

    pub async fn set_theme(&mut self, id: ThemeID) {
        info!("Set theme {id}");
        if let Ok(mut theme) = theming::THEME.write() {
            *theme = (*self.get_theme()).clone();
        }
        self.config
            .insert("system", "theme", Value::String(id.clone()));
        self.themes.set_theme(id).await;
    }
}
