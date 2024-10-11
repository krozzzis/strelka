use iced::widget::text_editor::{Catalog, Status, Style, StyleFn};
use iced::{Border, Color};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, _status: Status| Style {
            border: Border {
                color: Color::TRANSPARENT,
                ..Default::default()
            },
            background: Color::TRANSPARENT.into(),
            icon: theme.editor.text.into(),
            placeholder: theme.editor.text.into(),
            value: theme.editor.cursor.into(),
            selection: theme.editor.selection.into(),
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
