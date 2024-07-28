use iced::{
    border::Radius,
    widget::{button, component, text, Button, Component},
    Background, Border, Color, Element, Length, Padding, Renderer, Theme,
};

pub struct ListItem<Message>
where
    Message: Clone,
{
    title: String,
    tooltip: Option<String>,
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
}

impl<Message> Component<Message> for ListItem<Message>
where
    Message: Clone,
{
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme, Renderer> {
        Button::new(text(self.title.clone()))
            .on_press_maybe(self.on_click.clone())
            .width(Length::Fill)
            .padding(Padding::new(4.0).left(24.0))
            .style(move |_theme: &Theme, status| match status {
                button::Status::Active | button::Status::Disabled => button::Style {
                    background: None,
                    ..Default::default()
                },

                button::Status::Hovered | button::Status::Pressed => button::Style {
                    background: Some(Color::new(0.85, 0.85, 0.85, 1.0).into()),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(4.0),
                    },
                    ..Default::default()
                },
            })
            .into()
    }
}

impl<'a, Message> From<ListItem<Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    fn from(value: ListItem<Message>) -> Self {
        component(value)
    }
}
