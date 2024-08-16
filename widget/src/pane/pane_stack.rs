use core::pane::{Pane, PaneModel};

use iced::{widget::Space, Element, Length};
use theming::Theme;

use crate::{
    container::background,
    pane::new_document::{self, new_document_pane},
};

pub enum Message {
    NewDocument(new_document::Message),
}

pub fn pane_stack(model: &PaneModel) -> Element<'_, Message, Theme> {
    if let Some(pane) = model.get_open() {
        match *pane {
            Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
            Pane::NewDocument => new_document_pane().map(Message::NewDocument),
            Pane::Editor(_) => todo!(),
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    }
}
