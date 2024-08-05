mod color;
mod styles;
pub mod theme;

use iced::{
    border::Radius,
    widget::{button, container},
    Border, Color,
};

use crate::theming;

pub const FALLBACK: Theme = Theme {
    background: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    background2: Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    },
    surface: Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    text: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    subtext: Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    },
    selected: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    primary: Color {
        r: 0.7,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    },
    border_color: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    element_radius: 4.0,
    element_padding: 4.0,
    theme: theming::theme::FALLBACK,
};

#[derive(Debug, Clone)]
pub struct Theme<'a> {
    pub background: Color,
    pub background2: Color,
    pub surface: Color,
    pub text: Color,
    pub subtext: Color,
    pub selected: Color,
    pub primary: Color,
    pub border_color: Color,
    pub element_radius: f32,
    pub element_padding: f32,
    pub theme: theming::theme::Theme<'a>,
}

impl<'a> Theme<'a> {
    /// Dark theme from material ui
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb8(48, 52, 70),
            background2: Color::from_rgb8(35, 38, 52),
            surface: Color::from_rgb8(65, 69, 89),
            text: Color::from_rgb8(198, 208, 245),
            subtext: Color::from_rgb8(165, 173, 206),
            selected: Color::from_rgb8(98, 104, 128),
            primary: Color::from_rgb8(242, 213, 207),
            border_color: Color::from_rgb8(115, 121, 148),
            element_radius: 4.0,
            element_padding: 4.0,
            theme: theming::theme::Theme::default(),
        }
    }

    pub fn from_theme(theme: theme::Theme<'a>) -> Self {
        Self {
            background: theme.generic.background.into(),
            text: theme.generic.text.into(),
            theme,
            ..Default::default()
        }
    }

    pub fn container(&self) -> container::Style {
        container::Style {
            background: Some(self.background.into()),
            text_color: Some(self.text),
            ..Default::default()
        }
    }

    pub fn container2(&self) -> container::Style {
        container::Style {
            background: Some(self.background2.into()),
            text_color: Some(self.text),
            ..Default::default()
        }
    }

    pub fn container_with_radius(&self) -> container::Style {
        container::Style {
            background: Some(self.background.into()),
            text_color: Some(self.text),
            border: Border {
                radius: Radius::new(self.element_radius),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn text_button(&self) -> impl Fn(&iced::Theme, button::Status) -> button::Style + '_ {
        move |_, status| match status {
            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(self.theme.text_button.hover.background.into()),
                text_color: self.theme.text_button.hover.text.into(),
                ..Default::default()
            },

            button::Status::Disabled | button::Status::Active => button::Style {
                background: Some(self.theme.text_button.active.background.into()),
                text_color: self.theme.text_button.active.text.into(),
                ..Default::default()
            },
        }
    }
}

impl<'a> Default for Theme<'a> {
    /// Light theme from Catppuccin Latte
    fn default() -> Self {
        Self {
            background: Color::from_rgb8(239, 241, 245),
            background2: Color::from_rgb8(220, 224, 232),
            surface: Color::from_rgb8(204, 208, 218),
            text: Color::from_rgb8(76, 79, 105),
            subtext: Color::from_rgb8(92, 95, 119),
            selected: Color::from_rgb8(156, 160, 176),
            primary: Color::from_rgb8(220, 138, 120),
            border_color: Color::from_rgb8(124, 127, 147),
            element_radius: 4.0,
            element_padding: 4.0,
            theme: theming::theme::Theme::default(),
        }
    }
}
