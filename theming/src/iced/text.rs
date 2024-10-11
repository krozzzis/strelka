use iced::widget::text::{Catalog, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| Style {
            color: Some(theme.generic.text.into()),
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
