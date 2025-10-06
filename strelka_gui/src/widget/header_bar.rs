use iced::widget::{MouseArea, Space, Stack, button, container, row, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

pub fn header_bar<'a>() -> Element<'a, Message> {
    // Кнопки управления
    let close_btn = button(text("×")).on_press("close_window".into()).padding(4);

    let minimize_btn = button(text("–"))
        .on_press("collapse_window".into())
        .padding(4);

    let maximize_btn = button(text("▢"))
        .on_press("toggle_maximize_window".into())
        .padding(4);

    let drag_area = MouseArea::new(
        container(Space::with_width(Length::Fill))
            .style(iced::widget::container::secondary)
            .height(Length::Fill),
    )
    .on_press("start_drag_window".into())
    .on_release("end_drag_window".into());

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
