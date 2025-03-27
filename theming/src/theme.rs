use std::{borrow::Cow, path::Path};

use crate::styles::{
    button::Button, context_menu::ContextMenu, editor::Editor, file_explorer::FileExplorer,
    generic::Generic, list::List, list_item::ListItem, notification::Notification,
    notification_list::NotificationList, tab::Tab, tab_bar::TabBar,
};

#[cfg(feature = "iced")]
use iced::application::{Appearance, DefaultStyle};

#[cfg(feature = "load")]
use serde::{Deserialize, Serialize};

pub const FALLBACK: Theme = Theme {
    primary_button: Button::FALLBACK,
    secondary_button: Button::FALLBACK,
    text_button: Button::FALLBACK,
    tab: Tab::FALLBACK,
    tab_bar: TabBar::FALLBACK,
    list_item: ListItem::FALLBACK,
    list: List::FALLBACK,
    editor: Editor::FALLBACK,
    notification: Notification::FALLBACK,
    notification_list: NotificationList::FALLBACK,
    context_menu: ContextMenu::FALLBACK,
    file_explorer: FileExplorer::FALLBACK,
    generic: Generic::FALLBACK,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Theme {
    // Buttons
    pub primary_button: Button,
    pub secondary_button: Button,
    pub text_button: Button,

    // Tabs
    pub tab: Tab,
    pub tab_bar: TabBar,

    // List
    pub list_item: ListItem,
    pub list: List,

    // Notifications
    pub notification: Notification,
    pub notification_list: NotificationList,

    pub editor: Editor,
    pub context_menu: ContextMenu,
    pub file_explorer: FileExplorer,
    pub generic: Generic,
}

impl Theme {
    #[cfg(feature = "load")]
    pub async fn from_file(path: impl AsRef<async_std::path::Path>) -> Result<Theme, String> {
        let text = async_std::fs::read_to_string(path)
            .await
            .map_err(|e| e.to_string())?;
        let theme: Theme = toml::from_str(&text).map_err(|e| e.to_string())?;
        Ok(theme)
    }
}

#[cfg(feature = "iced")]
impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.generic.background.into(),
            text_color: self.generic.text.into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        FALLBACK
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Info<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}
