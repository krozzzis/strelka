use iced::{
    border::Radius,
    widget::{button, Button},
    Border, Color, Element,
};

use crate::theming::Theme;

pub fn primary_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Button<'a, Message> {
    Button::new(content).style(|_, status: button::Status| match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(theme.primary_button.hover.background.into()),
            text_color: theme.primary_button.hover.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },

        button::Status::Active | button::Status::Disabled => button::Style {
            background: Some(theme.primary_button.active.background.into()),
            text_color: theme.primary_button.active.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },
    })
}

pub fn secondary_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Button<'a, Message> {
    Button::new(content).style(|_, status: button::Status| match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(theme.secondary_button.hover.background.into()),
            text_color: theme.secondary_button.hover.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },

        button::Status::Active | button::Status::Disabled => button::Style {
            background: Some(theme.secondary_button.active.background.into()),
            text_color: theme.secondary_button.active.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },
    })
}

pub fn text_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Button<'a, Message> {
    Button::new(content).style(|_, status: button::Status| match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(theme.text_button.hover.background.into()),
            text_color: theme.text_button.hover.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },

        button::Status::Active | button::Status::Disabled => button::Style {
            background: Some(theme.text_button.active.background.into()),
            text_color: theme.text_button.active.text.into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        },
    })
}
