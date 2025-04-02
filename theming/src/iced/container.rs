use strelka_core::{smol_str::SmolStr, Color};

use iced::widget::container::{Catalog, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn transparent(theme: &Theme) -> Style {
    Style {
        text_color: Some(
            theme
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        ),
        background: None,
        ..Default::default()
    }
}

pub fn background(theme: &Theme) -> Style {
    Style {
        text_color: Some(
            theme
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        ),
        background: Some(
            theme
                .get_color_or_default(&SmolStr::new_static("container.background"), Color::WHITE)
                .into(),
        ),
        ..Default::default()
    }
}

pub fn background2(theme: &Theme) -> Style {
    Style {
        text_color: Some(
            theme
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        ),
        background: Some(
            theme
                .get_color_or_default(&SmolStr::new_static("container.background2"), Color::WHITE)
                .into(),
        ),
        ..Default::default()
    }
}
