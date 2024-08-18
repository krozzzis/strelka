use iced::{
    widget::{container, text, MouseArea, Row},
    Element, Length, Padding,
};
use theming::{iced::container::background, theme, Theme};

use crate::{button::a, Label};

pub struct Tab<Message: Clone> {
    pub label: Option<Label>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_middle_click: Option<Message>,
}

pub fn tab<'a, Message: Clone + 'a>(tab: &Tab<Message>) -> Element<'a, Message, Theme> {
    let title = tab.label.clone().unwrap_or_default();

    let selected = tab.selected;
    let btn = a::Button::new(container(text(title)).padding(Padding::from(0.0).left(8.0).top(6.0)))
        .selected(selected)
        .on_press_maybe(tab.on_click.clone())
        .height(theme!(tab.height));

    if let Some(message) = &tab.on_middle_click {
        let middle_clickable = MouseArea::new(btn).on_middle_release(message.clone());
        middle_clickable.into()
    } else {
        btn.into()
    }
}

pub fn tab_bar<'a, Message: Clone + 'a>(tabs: Vec<Tab<Message>>) -> Element<'a, Message, Theme> {
    let tabs = tabs.iter().map(tab);

    let content = container(Row::with_children(tabs).spacing(theme!(tab_bar.spacing)))
        .padding(theme!(tab_bar.padding));

    container(content)
        .width(Length::Fill)
        .height(theme!(tab_bar.height))
        .style(|theme: &Theme| container::Style {
            background: Some(theme.tab_bar.background.into()),
            ..background(theme)
        })
        .into()
}
