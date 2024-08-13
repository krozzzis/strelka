use iced::{
    widget::{container::Style, Container},
    Element,
};

use theming::Theme;

pub fn background<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Container<'a, Message> {
    Container::new(content).style(|_| Style {
        background: Some(theme.generic.background.into()),
        text_color: Some(theme.generic.text.into()),
        ..Default::default()
    })
}

pub fn background2<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Container<'a, Message> {
    Container::new(content).style(|_| Style {
        background: Some(theme.generic.background2.into()),
        text_color: Some(theme.generic.text.into()),
        ..Default::default()
    })
}
