use std::{borrow::Cow, sync::Arc};

use iced::{
    border::Radius,
    widget::{
        button, component,
        text::{self, Fragment, IntoFragment, Span},
        Component, Text,
    },
    Border, Element,
};

use crate::theme::Theme;

pub struct Tab<'a, Message> {
    pub label: Cow<'a, str>,
    pub theme: Option<&'a Theme>,
    pub on_click: Option<Message>,
    pub selected: bool,
}

impl<'a, Message> Tab<'a, Message> {
    pub fn new(label: Cow<'a, str>) -> Self {
        Self {
            label,
            theme: None,
            on_click: None,
            selected: false,
        }
    }

    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
        button(Text::new(self.label.clone()))
            .on_press_maybe(self.on_click.clone())
            .style(move |_, status| {
                let bg_normal = self
                    .theme
                    .map_or(Theme::default().background, move |theme| theme.background);
                let bg_selected = self
                    .theme
                    .map_or(Theme::default().selected, move |theme| theme.selected);
                let text_color = self
                    .theme
                    .map_or(Theme::default().text, move |theme| theme.text);
                let border_radius = self
                    .theme
                    .map_or(Theme::default().element_radius, move |theme| {
                        theme.element_radius
                    });
                let border_color = self
                    .theme
                    .map_or(Theme::default().border_color, move |theme| {
                        theme.border_color
                    });
                match status {
                    button::Status::Hovered | button::Status::Pressed => button::Style {
                        background: Some(bg_selected.into()),
                        text_color,
                        border: Border {
                            color: border_color,
                            width: 1.0,
                            radius: Radius {
                                top_left: border_radius,
                                top_right: border_radius,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                        },
                        ..Default::default()
                    },
                    button::Status::Active | button::Status::Disabled => button::Style {
                        background: Some(if self.selected {
                            bg_selected.into()
                        } else {
                            bg_normal.into()
                        }),
                        text_color,
                        border: Border {
                            color: border_color,
                            width: 1.0,
                            radius: Radius {
                                top_left: border_radius,
                                top_right: border_radius,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                        },
                        ..Default::default()
                    },
                }
            })
            .into()
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
