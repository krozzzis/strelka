use core::Color;

use iced_core::Border;
use iced_widget::scrollable::{Catalog, Rail, Scroller, Status, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn default(theme: &Theme, _status: Status) -> Style {
    Style {
        container: transparent_container(theme),
        vertical_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                color: theme.generic.text.into(),
                border: Border::default(),
            },
        },
        horizontal_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                color: theme.generic.text.into(),
                border: Border::default(),
            },
        },
        gap: None,
    }
}

fn transparent_container(theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(theme.generic.text.into()),
        background: None,
        ..Default::default()
    }
}
