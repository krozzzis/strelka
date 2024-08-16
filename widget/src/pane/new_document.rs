use iced::{widget::center, Element};
use theming::Theme;

use crate::{button::text_button, container::background};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PickFile,
}

pub fn new_document_pane<'a>() -> Element<'a, Message, Theme> {
    background(center(
        text_button("Open file Ctrl+O").on_press(Message::PickFile),
    ))
    .into()
}
