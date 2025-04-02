use strelka_core::{smol_str::SmolStr, Color};

use iced::widget::text::{Catalog, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| Style {
            color: Some(
                theme
                    .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                    .into(),
            ),
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
