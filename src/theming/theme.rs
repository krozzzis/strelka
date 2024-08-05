use std::{borrow::Cow, path::PathBuf};

use crate::theming::styles::{
    button::Button, context_menu::ContextMenu, editor::Editor, generic::Generic, list::List,
    list_item::ListItem, notification::Notification, notification_list::NotificationList, tab::Tab,
    tab_bar::TabBar,
};

use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};

pub const FALLBACK: Theme = Theme {
    info: Info {
        name: Cow::Borrowed("fallback"),
        description: Cow::Borrowed("Fallback theme"),
    },
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
    generic: Generic::FALLBACK,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme<'a> {
    pub info: Info<'a>,

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
    pub generic: Generic,
}

impl<'a> Default for Theme<'a> {
    fn default() -> Self {
        FALLBACK
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Info<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

pub async fn from_file<'a>(path: PathBuf) -> Result<Theme<'a>, String> {
    let text = tokio::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .await?;
    let theme = toml::from_str(&text).map_err(|e| e.to_string())?;
    Ok(theme)
}
