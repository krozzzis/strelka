use iced::{
    widget::{column, container},
    Element, Length,
};

use theming::{theme, Theme};

pub fn list<'a, Message: 'a>(
    items: Vec<Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    container(
        column(items)
            .spacing(theme!(list.spacing))
            .width(Length::Fill)
            .height(Length::Shrink),
    )
    .width(Length::Fill)
    .into()
}
