use core::{
    document::DocumentId,
    pane::{Pane, PaneId},
    State,
};

use iced::{
    widget::{column, text_editor::Content, Space},
    Element, Length,
};
use theming::Theme;

use crate::{
    container::background,
    pane::{
        new_document::{self, new_document_pane},
        text_editor,
    },
    tab::{tab_bar, Tab},
    util::filename,
    Label,
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenPane(PaneId),
    ClosePane(PaneId),
    NewDocument(new_document::Message),
    TextEditor(DocumentId, text_editor::Message),
}

pub fn pane_stack(state: State<'_, Content>) -> Element<'_, Message, Theme> {
    let open = state.panes.get_open_id().unwrap_or(&0);

    let tabs: Vec<Tab<Message>> = state
        .panes
        .list()
        .iter()
        .map(|(id, pane)| {
            let title: Option<Label> = match **pane {
                Pane::Empty => None,
                Pane::NewDocument => Some("New tab".into()),
                Pane::Editor(id) => state
                    .documents
                    .get(&id)
                    .map(|handler| filename(handler.path.clone()).unwrap_or_default().into()),
            };

            Tab {
                label: title,
                selected: *id == open,
                on_click: Some(Message::OpenPane(**id)),
                on_close: Some(Message::ClosePane(**id)),
                on_middle_click: Some(Message::ClosePane(**id)),
            }
        })
        .collect();

    let tab_bar = tab_bar(tabs);

    let pane = if let Some(pane) = state.panes.get_open() {
        match *pane {
            Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
            Pane::NewDocument => new_document_pane().map(Message::NewDocument),
            Pane::Editor(id) => text_editor::text_editor(id, state)
                .map(move |action| Message::TextEditor(id, action)),
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}
