use iced::{widget::Container, Element};
use strelka_core::Theme;

pub fn background<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Container<'a, Message, Theme> {
    Container::new(content).style(strelka_core::iced::container::background)
}

pub fn background2<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Container<'a, Message, Theme> {
    Container::new(content).style(strelka_core::iced::container::background2)
}
