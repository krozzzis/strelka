use core::{
    buffer::{Buffer, FormattedBuffer},
    document::DocumentId,
    pane::{Pane, PaneId},
    State,
};

use iced::{
    widget::{column, svg, text_editor::Content, Space},
    Element, Length,
};
use theming::Theme;

use crate::{
    buffer::buffer,
    container::background,
    pane::{
        new_document::{self, new_document_pane},
        text_editor,
    },
    tab::{tab_bar, Tab},
    util::filename,
    Label,
};

lazy_static::lazy_static! {
    pub static ref BUFFER: Buffer = Buffer::new("Hello\nAboba");
    pub static ref FORMATTED: FormattedBuffer = FormattedBuffer::from_buffer(&BUFFER);
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenPane(PaneId),
    ClosePane(PaneId),
    NewPane(Pane),
    NewDocument(new_document::Message),
    TextEditor(DocumentId, text_editor::Message),
}

pub fn pane_stack(state: State<'_, Content>) -> Element<'_, Message, Theme> {
    let open = state.panes.get_open_id().unwrap_or(&0);

    let mut tabs: Vec<Tab<Message>> = state
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
                Pane::Buffer => Some("Buffer tab (EXPERIMENTAL)".into()),
            };

            Tab {
                label: title,
                icon: None,
                selected: *id == open,
                on_click: Some(Message::OpenPane(**id)),
                on_close: Some(Message::ClosePane(**id)),
                on_middle_click: Some(Message::ClosePane(**id)),
            }
        })
        .collect();

    let new_tab_button = Tab {
        label: None,
        icon: Some(svg::Handle::from_path("./images/plus.svg")),
        selected: false,
        on_click: Some(Message::NewPane(Pane::NewDocument)),
        on_close: None,
        on_middle_click: None,
    };

    tabs.push(new_tab_button);

    let tab_bar = tab_bar(tabs);

    let pane = if let Some(pane) = state.panes.get_open() {
        match *pane {
            Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
            Pane::NewDocument => new_document_pane().map(Message::NewDocument),
            Pane::Editor(id) => text_editor::text_editor(id, state)
                .map(move |action| Message::TextEditor(id, action)),
            Pane::Buffer => background(buffer(&FORMATTED)).into(),
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}
