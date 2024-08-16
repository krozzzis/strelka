use core::pane::{Pane, PaneModel};

use iced::{
    widget::{column, Space},
    Element, Length,
};
use theming::Theme;

use crate::{
    container::background,
    pane::new_document::{self, new_document_pane},
    tab::{tab_bar, Tab},
    Label,
};

#[derive(Debug, Clone)]
pub enum Message {
    NewDocument(new_document::Message),
}

pub fn pane_stack(model: &PaneModel) -> Element<'_, Message, Theme> {
    let titles: Vec<Option<Label>> = model
        .list()
        .iter()
        .map(|(_id, pane)| match **pane {
            Pane::Empty => None,
            Pane::NewDocument => Some("New tab".into()),
            Pane::Editor(_) => Some("Some document".into()),
        })
        .collect();

    let tabs: Vec<Tab<Message>> = titles
        .iter()
        .map(|title| Tab {
            label: title.clone(),
            on_click: None,
        })
        .collect();

    let tab_bar = tab_bar(tabs);

    let pane = if let Some(pane) = model.get_open() {
        match *pane {
            Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
            Pane::NewDocument => new_document_pane().map(Message::NewDocument),
            Pane::Editor(_) => todo!(),
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}
