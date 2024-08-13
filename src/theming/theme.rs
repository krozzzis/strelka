use std::{borrow::Cow, path::PathBuf};

use crate::theming::styles::{
    button::Button, context_menu::ContextMenu, editor::Editor, file_explorer::FileExplorer,
    generic::Generic, list::List, list_item::ListItem, notification::Notification,
    notification_list::NotificationList, tab::Tab, tab_bar::TabBar,
};

use iced::{
    border::Radius,
    futures::TryFutureExt,
    widget::{self, button},
    Border, Shadow,
};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn text_button(&self) -> impl Fn(&iced::Theme, button::Status) -> button::Style + '_ {
        move |_, status| match status {
            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(self.text_button.hover.background.into()),
                text_color: self.text_button.hover.text.into(),
                ..Default::default()
            },

            button::Status::Disabled | button::Status::Active => button::Style {
                background: Some(self.text_button.active.background.into()),
                text_color: self.text_button.active.text.into(),
                ..Default::default()
            },
        }
    }

    pub async fn from_file(path: PathBuf) -> Result<Theme, String> {
        let text = tokio::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .await?;
        let theme = toml::from_str(&text).map_err(|e| e.to_string())?;
        Ok(theme)
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
