use std::borrow::Cow;

use iced::{
    border::Radius,
    widget::{button, component, container, Component, MouseArea, Text},
    Border, Element, Length, Size,
};

use crate::theming::Theme;

pub struct Tab<'a, Message> {
    pub label: Cow<'a, str>,
    pub theme: Option<&'a Theme>,
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
                match status {
                    button::Status::Hovered | button::Status::Pressed => button::Style {
                        background: Some(bg_selected.into()),
                        text_color,
                        border: Border {
                            radius: Radius {
                                top_left: border_radius,
                                top_right: border_radius,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                            ..Default::default()
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
                            radius: Radius {
                                top_left: border_radius,
                                top_right: border_radius,
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
