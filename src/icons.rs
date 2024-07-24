use iced::widget::svg::Handle;

pub const ACTION_ICON_PATH: &str = "./content/action.svg";
pub const SETTINGS_ICON_PATH: &str = "./content/settings.svg";
pub const PLUGINS_ICON_PATH: &str = "./content/plugins.svg";

pub struct IconStorage {
    pub action: Handle,
    pub settings: Handle,
    pub plugins: Handle,
}

impl IconStorage {
    pub fn new() -> Self {
        Self {
            action: Handle::from_path(ACTION_ICON_PATH),
            settings: Handle::from_path(SETTINGS_ICON_PATH),
            plugins: Handle::from_path(PLUGINS_ICON_PATH),
        }
    }
}
