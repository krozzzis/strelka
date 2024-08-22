use core::buffer::FormattedBuffer;

use iced::widget::{text, Column};
use theming::Theme;

pub fn buffer<'a, Message: Clone + 'a>(
    buffer: &'a FormattedBuffer,
) -> iced::Element<'a, Message, Theme> {
    let elements: Vec<iced::Element<'_, Message, Theme>> = buffer
        .elements
        .iter()
        .map(|element| match element {
            core::buffer::Element::Span(label, attrs) => text(label)
                .size(attrs.font_size)
                .color(attrs.font_color)
                .into(),
        })
        .collect();

    let column = Column::with_children(elements);
    column.into()
}
