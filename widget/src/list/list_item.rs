use iced::{
    border::Radius,
    widget::{button, component, text, Button, Component},
    Border, Color, Element, Length, Padding,
};

use theming::{self, Theme};

pub struct ListItem<Message>
where
    Message: Clone,
{
    title: String,
    tooltip: Option<String>,
    selected: bool,
    on_click: Option<Message>,
}

impl<Message> ListItem<Message>
where
    Message: Clone,
{
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            tooltip: None,
            selected: false,
            on_click: None,
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
}

impl<Message> Component<Message, Theme> for ListItem<Message>
where
    Message: Clone,
{
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme> {
        Button::new(
            text(self.title.clone())
                .size(14.0)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.list_item.active.text.into()),
                }),
        )
        .on_press_maybe(self.on_click.clone())
        .width(Length::Fill)
        .padding(Padding::new(4.0).left(24.0))
        .style(|theme: &Theme, status| match status {
            button::Status::Active | button::Status::Disabled => button::Style {
                background: if self.selected {
                    Some(theme.list_item.selected.background.into())
                } else {
                    Some(theme.list_item.active.background.into())
                },
                text_color: theme.list_item.active.text.into(),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(theme.list_item.active.radius),
                },
                ..Default::default()
            },

            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(theme.list_item.hover.background.into()),
                text_color: theme.list_item.hover.text.into(),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(theme.list_item.hover.radius),
                },
                ..Default::default()
            },
        })
        .into()
    }
}

impl<'a, Message> From<ListItem<Message>> for Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    fn from(value: ListItem<Message>) -> Self {
        component(value)
    }
}
