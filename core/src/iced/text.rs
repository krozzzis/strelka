use crate::{smol_str::SmolStr, Color, Theme};

use iced_widget::text::{Catalog, Style, StyleFn};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| Style {
            color: Some(
                theme
                    .inner
                    .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                    .into(),
            ),
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
