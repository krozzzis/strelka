use crate::{smol_str::SmolStr, Color, Theme};

use iced_widget::container::{Catalog, Style, StyleFn};

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
                .inner
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
                .inner
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        ),
        background: Some(
            theme
                .inner
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
                .inner
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        ),
        background: Some(
            theme
                .inner
                .get_color_or_default(&SmolStr::new_static("container.background2"), Color::WHITE)
                .into(),
        ),
        ..Default::default()
    }
}
