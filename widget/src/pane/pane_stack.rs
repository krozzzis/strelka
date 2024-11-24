use core::{
    buffer::{Buffer, FormattedBuffer},
    pane::{Pane, PaneId, VisiblePaneModel},
};

use iced::{
    widget::{center, column, svg, Space},
    Element, Length,
};
use theming::Theme;

use crate::{
    buffer::buffer,
    container::background,
    pane::new_document::{self, new_document_pane},
    tab::{tab_bar, Tab},
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
    None,
}

pub fn pane_stack<'a>(model: VisiblePaneModel) -> Element<'a, Message, Theme> {
    let mut tabs: Vec<Tab<Message>> = model
        .panes
        .iter()
        .map(|(id, pane)| {
            let title: Option<Label> = match pane {
                Pane::Empty => None,
                Pane::NewDocument => Some("New tab".into()),
                Pane::Editor(_doc_id) => Some("File tab".into()),
                Pane::Buffer => Some("Buffer tab (EXPERIMENTAL)".into()),
                Pane::Config => Some("Config viewer".into()),
            };

            Tab {
                label: title,
                icon: None,
                selected: Some(*id) == model.opened,
                on_click: Some(Message::OpenPane(*id)),
                on_close: Some(Message::ClosePane(*id)),
                on_middle_click: Some(Message::ClosePane(*id)),
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

    let pane = if let Some(id) = model.opened {
        if let Some((_id, pane)) = model.panes.iter().find(|(x, _pane)| *x == id) {
            match pane {
                Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
                Pane::NewDocument => new_document_pane().map(Message::NewDocument),
                Pane::Editor(_id) => background(center("file buffer")).into(),
                Pane::Buffer => background(buffer(&FORMATTED)).into(),
                Pane::Config => background(center("config buffer")).into(),
            }
        } else {
            Space::new(Length::Fill, Length::Fill).into()
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}
