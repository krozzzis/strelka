use std::borrow::Cow;
use std::sync::Arc;

use iced::{
    widget::{container, row, Container},
    Element, Length, Padding,
};

use crate::tabs::Tab;
use theming::Theme;

pub fn tab_bar<'a, Message: 'a + Clone>(
    labels: Vec<(Arc<String>, Message, Option<Message>)>,
    selected: Option<usize>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let tabs: Vec<Element<'_, Message>> = labels
        .into_iter()
        .enumerate()
        .map(|(id, (label, click, close))| {
            Tab::new(Cow::Owned(label.as_ref().clone()))
                .theme(theme)
                .on_click(click)
                .on_close_maybe(close)
                .selected(Some(id) == selected)
                .into()
        })
        .collect();

    Container::new(row(tabs).spacing(theme.tab_bar.spacing))
        .width(Length::Fill)
        .height(Length::Fixed(theme.tab.active.height))
        .padding(Padding::new(theme.tab_bar.padding).bottom(0.0))
        .style(move |_| container::Style {
            background: Some(theme.tab_bar.background.into()),
            ..Default::default()
        })
        .into()
}
