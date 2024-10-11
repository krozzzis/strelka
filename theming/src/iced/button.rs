use iced::widget::button::{Catalog, Status, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Hovered | Status::Pressed => Style {
            background: Some(theme.primary_button.hover.background.into()),
            text_color: theme.primary_button.hover.text.into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(theme.primary_button.active.background.into()),
            text_color: theme.primary_button.active.text.into(),
            ..Default::default()
        },
    }
}

pub fn secondary(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Hovered | Status::Pressed => Style {
            background: Some(theme.secondary_button.hover.background.into()),
            text_color: theme.secondary_button.hover.text.into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(theme.secondary_button.active.background.into()),
            text_color: theme.secondary_button.active.text.into(),
            ..Default::default()
        },
    }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Hovered | Status::Pressed => Style {
            background: Some(theme.text_button.hover.background.into()),
            text_color: theme.text_button.hover.text.into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(theme.text_button.active.background.into()),
            text_color: theme.text_button.active.text.into(),
            ..Default::default()
        },
    }
}
