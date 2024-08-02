use std::borrow::Cow;
use std::sync::Arc;

use iced::{
    widget::{container, row, Container},
    Element, Length, Padding,
};

use crate::{theme::Theme, widget::tabs::Tab};

pub fn tab_bar<'a, Message: 'a + Clone>(
    labels: Vec<(Arc<String>, Message)>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let tabs: Vec<Element<'_, Message>> = labels
        .into_iter()
        .map(|(label, msg)| {
            Tab::new(Cow::Owned(label.as_ref().clone()))
                .theme(theme)
                .on_click(msg)
                .into()
        })
        .collect();

    Container::new(row(tabs).spacing(4.0))
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(Padding::new(4.0).bottom(0.0))
        .style(move |_| container::Style {
            background: Some(theme.background2.into()),
            ..theme.container()
        })
        .into()
}
