use iced::{
    border::Radius,
    widget::{button, component, text, Button, Component},
    Border, Color, Element, Length, Padding,
};

use crate::theming::Theme;

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
        Button::new(
            text(self.title.clone())
                .size(14.0)
                .style(move |_| text::Style {
                    color: Some(self.theme.map_or(Theme::default().text, |theme| theme.text)),
                }),
        )
        .on_press_maybe(self.on_click.clone())
        .width(Length::Fill)
        .padding(Padding::new(4.0).left(24.0))
        .style(move |_, status| {
            let theme = self.theme.cloned().unwrap_or(Theme::default());

            match status {
                button::Status::Active | button::Status::Disabled => button::Style {
                    background: if self.selected {
                        Some(theme.selected.into())
                    } else {
                        None
                    },
                    text_color: theme.text,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(4.0),
                    },
                    ..Default::default()
                },

                button::Status::Hovered | button::Status::Pressed => button::Style {
                    background: Some(theme.selected.into()),
                    text_color: theme.text,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(4.0),
                    },
                    ..Default::default()
                },
            }
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
