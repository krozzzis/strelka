use iced::{
    border::Radius,
    widget::{
        button::{self, Status},
        container, text, Button, Row,
    },
    Alignment, Border, Element, Length, Padding,
};
use theming::{iced::container::background, theme, Theme};

use crate::Label;

pub struct Tab<Message: Clone> {
    pub label: Option<Label>,
    pub on_click: Option<Message>,
}

pub fn tab<'a, Message: Clone + 'a>(tab: &Tab<Message>) -> Element<'a, Message, Theme> {
    let title = tab.label.clone().unwrap_or_default();

    Button::new(text(title).align_y(Alignment::Center))
        .on_press_maybe(tab.on_click.clone())
        .height(theme!(tab.active.height))
        .style(|theme: &Theme, status: Status| match status {
            Status::Hovered | Status::Pressed => button::Style {
                background: Some(theme.tab.hover.background.into()),
                text_color: theme.tab.hover.text.into(),
                border: Border {
                    radius: Radius {
                        top_left: theme.tab.hover.radius,
                        top_right: theme.tab.hover.radius,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },

            Status::Active | Status::Disabled => button::Style {
                background: Some(theme.tab.active.background.into()),
                text_color: theme.tab.active.text.into(),
                border: Border {
                    radius: Radius {
                        top_left: theme.tab.active.radius,
                        top_right: theme.tab.active.radius,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .into()
}

pub fn tab_bar<'a, Message: Clone + 'a>(tabs: Vec<Tab<Message>>) -> Element<'a, Message, Theme> {
    let tabs = tabs.iter().map(tab);

    container(Row::with_children(tabs).spacing(theme!(tab_bar.spacing)))
        .padding(Padding::new(theme!(tab_bar.padding)).bottom(0.0))
        .width(Length::Fill)
        .height(theme!(tab_bar.padding) + theme!(tab.active.height))
        .style(|theme: &Theme| container::Style {
            background: Some(theme.tab_bar.background.into()),
            ..background(theme)
        })
        .into()
}
