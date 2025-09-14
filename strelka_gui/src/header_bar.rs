use iced::widget::{MouseArea, Space, Stack, button, container, row, text};
use iced::{Alignment, Element, Length};

use strelka_api::message::WindowMessage;

pub fn header_bar<'a>() -> Element<'a, WindowMessage> {
    // Кнопки управления
    let close_btn = button(text("×")).on_press(WindowMessage::Close).padding(4);

    let minimize_btn = button(text("–"))
        .on_press(WindowMessage::Collapse)
        .padding(4);

    let maximize_btn = button(text("▢"))
        .on_press(WindowMessage::ToggleMaximize)
        .padding(4);

    let drag_area = MouseArea::new(
        container(Space::with_width(Length::Fill))
            .style(iced::widget::container::secondary)
            .height(Length::Fill),
    )
    .on_press(WindowMessage::DragStart)
    .on_release(WindowMessage::DragEnd);

    Stack::with_children(vec![
        drag_area.into(),
        row![
            Space::with_width(Length::Fill),
            minimize_btn,
            maximize_btn,
            close_btn,
        ]
        .spacing(8.0)
        .padding(2.0)
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .into(),
    ])
    .height(Length::Fixed(32.0))
    .into()
}
