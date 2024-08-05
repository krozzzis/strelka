mod color;
mod theme;

use iced::{
    border::Radius,
    widget::{button, container},
    Border, Color,
};

#[derive(Clone)]
pub struct Theme {
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
}

impl Theme {
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

    pub fn transparent_button(
        &self,
    ) -> impl Fn(&iced::Theme, button::Status) -> button::Style + '_ {
        move |_, status| match status {
            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: None,
                text_color: self.primary,
                ..Default::default()
            },

            button::Status::Disabled | button::Status::Active => button::Style {
                background: None,
                text_color: self.text,
                ..Default::default()
            },
        }
    }
}

impl Default for Theme {
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
        }
    }
}
