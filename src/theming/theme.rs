use std::{borrow::Cow, path::PathBuf};

use crate::theming::styles::{
    button::Button, editor::Editor, generic::Generic, list::List, list_item::ListItem, notification::Notification, notification_list::NotificationList, tab::Tab, tab_bar::TabBar
};

use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};

pub const FALLBACK: Theme = Theme {
    info: Info {
        name: Cow::Borrowed("fallback"),
        description: Cow::Borrowed("Fallback theme"),
    },
    button: Button::FALLBACK,
    tab: Tab::FALLBACK,
    tab_bar: TabBar::FALLBACK,
    list_item: ListItem::FALLBACK,
    list: List::FALLBACK,
    editor: Editor::FALLBACK,
    notification: Notification::FALLBACK,
    notification_list: NotificationList::FALLBACK,
    generic: Generic::FALLBACK,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme<'a> {
    pub info: Info<'a>,
    pub button: Button,
    pub tab: Tab,
    pub tab_bar: TabBar,
    pub list_item: ListItem,
    pub list: List,
    pub editor: Editor,
    pub notification: Notification,
    pub notification_list: NotificationList,
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
