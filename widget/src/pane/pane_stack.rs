use core::pane::{Pane, PaneId, PaneModel};

use iced::{
    widget::{column, Space},
    Element, Length,
};
use theming::Theme;

use crate::{
    container::background,
    pane::new_document::{self, new_document_pane},
    tab::{tab_bar, Tab},
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenPane(PaneId),
    ClosePane(PaneId),
    NewDocument(new_document::Message),
}

pub fn pane_stack(model: &PaneModel) -> Element<'_, Message, Theme> {
    let tabs: Vec<Tab<Message>> = model
        .list()
        .iter()
        .map(|(id, pane)| {
            let title = match **pane {
                Pane::Empty => None,
                Pane::NewDocument => Some("New tab".into()),
                Pane::Editor(_) => Some("Some document".into()),
            };

            Tab {
                label: title,
                on_click: Some(Message::OpenPane(**id)),
                on_middle_click: Some(Message::ClosePane(**id)),
            }
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
