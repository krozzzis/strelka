use crate::{smol_str::SmolStr, Color, Theme};

use iced_widget::button::{Catalog, Status, Style, StyleFn};

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
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("button.hover.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(&SmolStr::new_static("button.hover.text"), Color::BLACK)
                .into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("button.active.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(&SmolStr::new_static("button.active.text"), Color::BLACK)
                .into(),
            ..Default::default()
        },
    }
}

pub fn secondary(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Hovered | Status::Pressed => Style {
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("secondary_button.hover.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(
                    &SmolStr::new_static("secondary_button.hover.text"),
                    Color::BLACK,
                )
                .into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("secondary_button.active.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(
                    &SmolStr::new_static("secondary_button.active.text"),
                    Color::BLACK,
                )
                .into(),
            ..Default::default()
        },
    }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Hovered | Status::Pressed => Style {
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("text_button.hover.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(&SmolStr::new_static("text_button.hover.text"), Color::BLACK)
                .into(),
            ..Default::default()
        },

        Status::Disabled | Status::Active => Style {
            background: Some(
                theme
                    .inner
                    .get_color_or_default(
                        &SmolStr::new_static("text_button.active.background"),
                        Color::WHITE,
                    )
                    .into(),
            ),
            text_color: theme
                .inner
                .get_color_or_default(
                    &SmolStr::new_static("text_button.active.text"),
                    Color::BLACK,
                )
                .into(),
            ..Default::default()
        },
    }
}
