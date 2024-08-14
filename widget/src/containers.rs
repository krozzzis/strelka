use iced::{
    widget::{container::Style, Container},
    Element,
};

use theming::Theme;

pub fn background<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Container<'a, Message, Theme> {
    Container::new(content).style(theming::iced::container::background)
}

pub fn background2<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Container<'a, Message, Theme> {
    Container::new(content).style(theming::iced::container::background2)
}
