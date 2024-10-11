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
        text_color: Some(theme.generic.text.into()),
        background: None,
        ..Default::default()
    }
}

pub fn background(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.generic.text.into()),
        background: Some(theme.generic.background.into()),
        ..Default::default()
    }
}

pub fn background2(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.generic.text.into()),
        background: Some(theme.generic.background2.into()),
        ..Default::default()
    }
}
