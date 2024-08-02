use iced::{
    widget::{button, container, row, Container},
    Element, Length, Padding,
};

use crate::{
    theme::Theme,
    widget::tabs::{tab, Tab},
};

pub fn tab_bar<'a, Message: 'a + Clone>(
    labels: Box<[String]>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let tabs: Vec<Element<'_, Message>> = labels
        .into_iter()
        .map(|label| Tab::new(label.clone()).theme(theme).into())
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
