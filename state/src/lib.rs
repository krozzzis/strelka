use config::Config;
use core::{document::DocumentStore, pane::PaneModel, smol_str::SmolStr, value::Value};
use iced::widget::text_editor::Content;
use log::info;
use theming::{
    catalog::{Catalog, ThemeID},
    Theme,
};

pub struct State {
    pub documents: DocumentStore<Content>,
    pub panes: PaneModel,
    pub themes: Catalog,
    pub config: Config,
}

impl State {
    pub fn get_theme(&self) -> Theme {
        let id = if let Some(Value::String(str)) = self.config.get("system", "theme") {
            SmolStr::new(str)
        } else {
            SmolStr::new("core.light")
        };

        info!("{id}");

        if let Some(theme) = self.themes.get_theme(id) {
            theme.clone()
        } else {
            Theme::default()
        }
    }

    pub fn set_theme(&mut self, id: ThemeID) {
        info!("Set theme {id}");
        self.config.insert("system", "theme", Value::String(id));
        if let Ok(mut theme) = theming::THEME.write() {
            *theme = self.get_theme();
        }
    }
}
