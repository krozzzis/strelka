use iced::{
    border::Radius,
    widget::{button, component, text, Button, Component},
    Border, Color, Element, Length, Padding,
};

use theming::{self, Theme};

pub struct ListItem<'a, Message>
where
    Message: Clone,
{
    title: String,
    tooltip: Option<String>,
    selected: bool,
    on_click: Option<Message>,
    theme: Option<&'a Theme>,
}

impl<'a, Message> ListItem<'a, Message>
where
    Message: Clone,
{
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            tooltip: None,
            selected: false,
            on_click: None,
            theme: None,
        }
    }

    pub fn tooltip(mut self, tooltip: String) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn tooltip_maybe(mut self, tooltip: Option<String>) -> Self {
        self.tooltip = tooltip;
        self
    }

    pub fn click(mut self, click: Message) -> Self {
        self.on_click = Some(click);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn theme(mut self, theme: Option<&'a Theme>) -> Self {
        self.theme = theme;
        self
    }
}

impl<'a, Message> Component<Message> for ListItem<'a, Message>
where
    Message: Clone,
{
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let fallback = &theming::FALLBACK;
        let theme = &self.theme.unwrap_or(fallback).list_item;

        Button::new(
            text(self.title.clone())
                .size(14.0)
                .style(move |_| text::Style {
                    color: Some(theme.active.text.into()),
                }),
        )
        .on_press_maybe(self.on_click.clone())
        .width(Length::Fill)
        .padding(Padding::new(4.0).left(24.0))
        .style(move |_, status| match status {
            button::Status::Active | button::Status::Disabled => button::Style {
                background: if self.selected {
                    Some(theme.selected.background.into())
                } else {
                    Some(theme.active.background.into())
                },
                text_color: theme.active.text.into(),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(theme.active.radius),
                },
                ..Default::default()
            },

            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(theme.hover.background.into()),
                text_color: theme.hover.text.into(),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(theme.hover.radius),
                },
                ..Default::default()
            },
        })
        .into()
    }
}

impl<'a, Message> From<ListItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: ListItem<'a, Message>) -> Self {
        component(value)
    }
}
