use std::borrow::Cow;

use iced::{
    border::Radius,
    widget::{button, component, container, Component, MouseArea, Text},
    Border, Element, Length, Size,
};

use crate::theming::{self, Theme};

pub struct Tab<'a, Message> {
    pub label: Cow<'a, str>,
    pub theme: Option<&'a Theme<'a>>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_close: Option<Message>,
}

impl<'a, Message> Tab<'a, Message> {
    pub fn new(label: Cow<'a, str>) -> Self {
        Self {
            label,
            theme: None,
            selected: false,
            on_click: None,
            on_close: None,
        }
    }

    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    pub fn on_close_maybe(mut self, message: Option<Message>) -> Self {
        self.on_close = message;
        self
    }
}

impl<'a, Message: 'a + Clone> Component<Message> for Tab<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let tab = button(container(Text::new(self.label.clone())).height(28.0))
            .on_press_maybe(self.on_click.clone())
            .style(move |_, status| {
                let fallback = &theming::FALLBACK;
                let theme = &self.theme.unwrap_or(fallback).theme.tab;

                match status {
                    button::Status::Hovered | button::Status::Pressed => button::Style {
                        background: Some(theme.hover.background.into()),
                        text_color: theme.hover.text.into(),
                        border: Border {
                            radius: Radius {
                                top_left: theme.hover.radius,
                                top_right: theme.hover.radius,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    button::Status::Active | button::Status::Disabled => button::Style {
                        background: Some(if self.selected {
                            theme.selected.background.into()
                        } else {
                            theme.active.background.into()
                        }),
                        text_color: theme.active.text.into(),
                        border: Border {
                            radius: Radius {
                                top_left: theme.active.radius,
                                top_right: theme.active.radius,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                }
            });

        let mut area = MouseArea::new(tab);

        if let Some(message) = self.on_close.clone() {
            area = area.on_middle_release(message);
        }

        area.into()
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        Size::new(Length::Fixed(28.0), Length::Shrink)
    }
}

impl<'a, Message> From<Tab<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: Tab<'a, Message>) -> Self {
        component(value)
    }
}
