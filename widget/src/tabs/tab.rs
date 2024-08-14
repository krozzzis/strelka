use std::borrow::Cow;

use iced::{
    border::Radius,
    widget::{button, center, component, Component, MouseArea, Text},
    Border, Element, Length, Size,
};

use theming::{self, theme, Theme};

pub struct Tab<'a, Message> {
    pub label: Cow<'a, str>,
    pub selected: bool,
    pub on_click: Option<Message>,
    pub on_close: Option<Message>,
}

impl<'a, Message> Tab<'a, Message> {
    pub fn new(label: Cow<'a, str>) -> Self {
        Self {
            label,
            selected: false,
            on_click: None,
            on_close: None,
        }
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

impl<'a, Message: 'a + Clone> Component<Message, Theme> for Tab<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme> {
        let tab = button(
            center(Text::new(self.label.clone()))
                .width(Length::Shrink)
                .height(theme!(tab.active.height)),
        )
        .on_press_maybe(self.on_click.clone())
        .style(|theme: &Theme, status| match status {
            button::Status::Hovered | button::Status::Pressed => button::Style {
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
            button::Status::Active | button::Status::Disabled => button::Style {
                background: Some(if self.selected {
                    theme.tab.selected.background.into()
                } else {
                    theme.tab.active.background.into()
                }),
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
        });

        let mut area = MouseArea::new(tab);

        if let Some(message) = self.on_close.clone() {
            area = area.on_middle_release(message);
        }

        area.into()
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        Size::new(Length::Fixed(theme!(tab.active.height)), Length::Shrink)
    }
}

impl<'a, Message> From<Tab<'a, Message>> for Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    fn from(value: Tab<'a, Message>) -> Self {
        component(value)
    }
}
