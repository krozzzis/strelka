use iced::{
    widget::{
        button::{self, Status},
        container, text, Button, MouseArea, Row,
    },
    Alignment, Border, Element, Length, Padding,
};
use theming::{iced::container::background, theme, Theme};

use crate::Label;

pub struct Tab<Message: Clone> {
    pub label: Option<Label>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_middle_click: Option<Message>,
}

pub fn tab<'a, Message: Clone + 'a>(tab: &Tab<Message>) -> Element<'a, Message, Theme> {
    let title = tab.label.clone().unwrap_or_default();

    let selected = tab.selected;
    let btn = Button::new(text(title).align_y(Alignment::Center))
        .on_press_maybe(tab.on_click.clone())
        .height(theme!(tab.active.height))
        .style(move |theme: &Theme, status: Status| match status {
            Status::Hovered | Status::Pressed => button::Style {
                background: Some(theme.tab.hover.background.into()),
                text_color: theme.tab.hover.text.into(),
                border: Border {
                    radius: theme.tab.hover.border.radius.clone().into(),
                    color: theme.tab.hover.border.color.into(),
                    width: theme.tab.hover.border.width,
                },
                ..Default::default()
            },

            _ if selected => button::Style {
                background: Some(theme.tab.selected.background.into()),
                text_color: theme.tab.selected.text.into(),
                border: Border {
                    radius: theme.tab.selected.border.radius.clone().into(),
                    color: theme.tab.selected.border.color.into(),
                    width: theme.tab.selected.border.width,
                },
                ..Default::default()
            },

            Status::Active | Status::Disabled => button::Style {
                background: Some(theme.tab.active.background.into()),
                text_color: theme.tab.active.text.into(),
                border: Border {
                    radius: theme.tab.active.border.radius.clone().into(),
                    color: theme.tab.active.border.color.into(),
                    width: theme.tab.active.border.width,
                },
                ..Default::default()
            },
        });

    if let Some(message) = &tab.on_middle_click {
        let middle_clickable = MouseArea::new(btn).on_middle_release(message.clone());
        middle_clickable.into()
    } else {
        btn.into()
    }
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
