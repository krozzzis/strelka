use iced::{
    widget::{center, column, text},
    Alignment, Element, Font, Length,
};
use theming::Theme;

use crate::{button::text_button, container::background};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PickFile,
}

pub fn new_document_pane<'a>() -> Element<'a, Message, Theme> {
    background(center(
        column![
            center(
                text("No file is open")
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .align_x(Alignment::Center)
                    .size(24.0)
            )
            .height(Length::Shrink),
            center(text_button("Open file Ctrl+O").on_press(Message::PickFile))
                .height(Length::Shrink),
        ]
        .spacing(16.0),
    ))
    .into()
}
