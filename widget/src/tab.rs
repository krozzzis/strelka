use core::Color;

use iced::{
    widget::{container, svg, text, MouseArea, Row, Space},
    Alignment, Element, Length, Padding,
};
use theming::{iced::container::background, theme, Theme};

use crate::{
    button::{a, secondary_button},
    Label,
};

pub struct Tab<Message: Clone> {
    pub label: Option<Label>,
    pub icon: Option<svg::Handle>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_close: Option<Message>,
    pub on_middle_click: Option<Message>,
}

pub fn tab<'a, Message: Clone + 'a>(tab: &Tab<Message>) -> Element<'a, Message, Theme> {
    let mut content = Vec::new();

    // Icon
    if let Some(handle) = tab.icon.clone() {
        let icon = container(
            svg::Svg::new(handle)
                .width(16.0)
                .height(16.0)
                .content_fit(iced::ContentFit::Fill),
        )
        .padding(4.0);
        content.push(icon.into())
    }

    // Label
    if let Some(label) = tab.label.clone() {
        content.push(text(label).width(Length::Fill).height(24.0).into());
    } else {
        content.push(Space::with_width(Length::Fill).into());
    }

    // Close button
    if let Some(message) = tab.on_close.clone() {
        content.push(
            secondary_button(
                svg::Svg::new("./images/close.svg").content_fit(iced::ContentFit::Fill),
            )
            .width(20.0)
            .height(20.0)
            .padding(1.0)
            .on_press(message)
            .into(),
        );
    }

    let mut btn = a::Button::new(
        container(Row::with_children(content).align_y(Alignment::Center))
            .padding(Padding::from(6.0)),
    )
    .selected(tab.selected)
    .on_press_maybe(tab.on_click.clone())
    .height(theme!(tab.height));

    // Change width if button icon only
    if tab.icon.is_some() && tab.label.is_none() && tab.on_close.is_none() {
        btn = btn.min_width(36.0);
    }

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
