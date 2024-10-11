use iced::widget::svg::{Catalog, Status, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, _status: Status| Style {
            color: Some(theme.generic.text.into()),
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
