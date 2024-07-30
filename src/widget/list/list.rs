use iced::{
    widget::{column, container},
    Border, Element, Length,
};

use crate::theme::Theme;

pub fn list<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    theme: Option<&'a Theme>,
) -> Element<'a, Message> {
    container(
        column(items)
            .spacing(4.0)
            .padding(8.0)
            .width(Length::Fill)
            .height(Length::Shrink),
    )
    .width(Length::Fill)
    .style(move |_| {
        let theme = theme.cloned().unwrap_or(Theme::default());
        container::Style {
            text_color: Some(theme.text),
            background: Some(theme.background.into()),
            ..Default::default()
        }
    })
    .into()
}
