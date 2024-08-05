use iced::{
    widget::{column, container},
    Element, Length,
};

use crate::theming::Theme;

pub fn list<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    container(
        column(items)
            .spacing(theme.theme.list.spacing)
            .width(Length::Fill)
            .height(Length::Shrink),
    )
    .width(Length::Fill)
    .into()
}
