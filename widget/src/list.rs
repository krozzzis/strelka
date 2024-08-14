use iced::{
    border::Radius,
    widget::{
        button::{Status, Style},
        container::{self, Container},
        text, Button, Column, Space,
    },
    Border, Element, Length,
};

use theming::{theme, Theme};

use crate::Label;

#[derive(Debug, Clone)]
pub struct List<Message> {
    pub elements: Vec<ListItem<Message>>,
}

impl<Message> List<Message>
where
    Message: Clone,
{
    pub fn with_elements(elements: Vec<ListItem<Message>>) -> Self {
        Self { elements }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        Column::with_children(self.elements.iter().map(|x| x.view(false)))
            .spacing(theme!(list.spacing))
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }
}

impl<Message> From<Vec<ListItem<Message>>> for List<Message> {
    fn from(elements: Vec<ListItem<Message>>) -> Self {
        Self { elements }
    }
}

impl<Message> Default for List<Message> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ListItem<Message> {
    Separator,
    TextButton(TextButton<Message>),
}

#[derive(Debug, Clone)]
pub struct TextButton<Message> {
    text: Label,
    on_click: Option<Message>,
}

impl<'a, Message> TextButton<Message>
where
    Message: Clone + 'a,
{
    pub fn new(text: impl Into<Label>) -> Self {
        Self {
            text: text.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn on_click_maybe(mut self, message: Option<Message>) -> Self {
        self.on_click = message;
        self
    }

    pub fn view(&self, selected: bool) -> Element<'_, Message, Theme> {
        Button::new(text(self.text.clone()))
            .width(Length::Fill)
            .on_press_maybe(self.on_click.clone())
            .style(move |theme: &Theme, status: Status| match status {
                Status::Hovered | Status::Pressed => Style {
                    background: Some(theme.list_item.selected.background.into()),
                    text_color: theme.list_item.selected.text.into(),
                    border: Border {
                        radius: Radius::from(theme.list_item.selected.radius),
                        ..Default::default()
                    },
                    ..Default::default()
                },

                Status::Disabled | Status::Active if selected => Style {
                    background: Some(theme.list_item.selected.background.into()),
                    text_color: theme.list_item.selected.text.into(),
                    border: Border {
                        radius: Radius::from(theme.list_item.selected.radius),
                        ..Default::default()
                    },
                    ..Default::default()
                },

                Status::Disabled | Status::Active => Style {
                    background: Some(theme.list_item.active.background.into()),
                    text_color: theme.list_item.active.text.into(),
                    border: Border {
                        radius: Radius::from(theme.list_item.active.radius),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            })
            .into()
    }
}

impl<Message> ListItem<Message>
where
    Message: Clone,
{
    pub fn view(&self, selected: bool) -> Element<'_, Message, Theme> {
        match self {
            ListItem::Separator => {
                Container::new(Space::with_width(Length::Fill).height(Length::Fixed(1.0)))
                    .style(|theme: &Theme| container::Style {
                        background: Some(theme.generic.text.into()),
                        ..Default::default()
                    })
                    .into()
            }
            ListItem::TextButton(btn) => btn.view(selected),
        }
    }
}
