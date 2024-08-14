use iced::{
    border::Radius,
    widget::{button, Button},
    Border, Element,
};

use theming::Theme;

pub fn primary_button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Button<'a, Message, Theme> {
    Button::new(content).style(theming::iced::button::primary)
}

pub fn secondary_button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Button<'a, Message, Theme> {
    Button::new(content).style(theming::iced::button::secondary)
}

pub fn text_button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Button<'a, Message, Theme> {
    Button::new(content).style(theming::iced::button::text)
}
