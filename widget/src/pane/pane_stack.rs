use core::{
    document::{DocumentId, DocumentStore},
    pane::{Pane, PaneId, PaneModel},
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
    Label,
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenPane(PaneId),
    ClosePane(PaneId),
    NewDocument(new_document::Message),
    TextEditor(DocumentId, text_editor::Message),
}

pub fn pane_stack<'a>(
    model: &'a PaneModel,
    documents: &'a DocumentStore<Content>,
) -> Element<'a, Message, Theme> {
    let tabs: Vec<Tab<Message>> = model
        .list()
        .iter()
        .map(|(id, pane)| {
            let title: Option<Label> = match **pane {
                Pane::Empty => None,
                Pane::NewDocument => Some("New tab".into()),
                Pane::Editor(id) => documents
                    .get(&id)
                    .map(|handler| handler.filename.clone().into()),
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
            Pane::Editor(id) => text_editor::text_editor(id, documents)
                .map(move |action| Message::TextEditor(id, action)),
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}
