use iced::{
    widget::{container, svg, text, MouseArea, Row},
    Alignment, Element, Length, Padding,
};
use theming::{iced::container::background, theme, Theme};

use crate::{
    button::{a, text_button},
    Label,
};

pub struct Tab<Message: Clone> {
    pub label: Option<Label>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_close: Option<Message>,
    pub on_middle_click: Option<Message>,
}

pub fn tab<'a, Message: Clone + 'a>(tab: &Tab<Message>) -> Element<'a, Message, Theme> {
    let title = tab.label.clone().unwrap_or_default();

    let selected = tab.selected;

    let mut content = vec![text(title).width(Length::Fill).height(24.0).into()];
    if let Some(message) = tab.on_close.clone() {
        content.push(
            text_button(svg::Svg::new("./images/close.svg").content_fit(iced::ContentFit::Fill))
                .width(20.0)
                .height(20.0)
                .on_press(message)
                .padding(1.0)
                .into(),
        );
    }

    let btn = a::Button::new(
        container(Row::with_children(content).align_y(Alignment::Center))
            .padding(Padding::from(0.0).left(12.0).top(6.0).right(10.0)),
    )
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
