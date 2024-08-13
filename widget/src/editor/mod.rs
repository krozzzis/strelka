use iced::{
    border::Radius,
    widget::{
        center, component, container,
        text_editor::{self, Content},
        Component, Container, TextEditor,
    },
    Border, Color, Element, Length, Pixels,
};

use theming::{self, Theme};

/// Text editor widget
pub struct NoteEditor<'a, Message> {
    content: &'a Content,
    theme: Option<&'a Theme>,
    on_action: Box<dyn Fn(text_editor::Action) -> Message>,
}

impl<'a, Message> NoteEditor<'a, Message> {
    pub fn new<F>(content: &'a Content, on_action: F) -> Self
    where
        F: 'static + Fn(text_editor::Action) -> Message,
    {
        Self {
            content,
            theme: None,
            on_action: Box::new(on_action),
        }
    }

    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl<'a, Message> Component<Message> for NoteEditor<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let fallback = &theming::FALLBACK;
        let theme = &self.theme.unwrap_or(fallback).editor;

        let editor = Container::new(center(
            Container::new(
                TextEditor::new(self.content)
                    .on_action(&self.on_action)
                    .height(Length::Fill)
                    .size(16.0)
                    .style(move |_, _status| text_editor::Style {
                        border: Border {
                            color: Color::TRANSPARENT,
                            ..Default::default()
                        },
                        background: Color::TRANSPARENT.into(),
                        icon: theme.text.into(),
                        placeholder: theme.text.into(),
                        value: theme.cursor.into(),
                        selection: theme.selection.into(),
                    }),
            )
            .padding(theme.padding)
            .max_width(Pixels::from(700.0))
            .style(move |_| container::Style {
                background: Some(theme.background2.into()),
                text_color: Some(theme.text.into()),
                border: Border {
                    radius: Radius::from(theme.radius),
                    ..Default::default()
                },
                ..Default::default()
            }),
        ))
        .padding(theme.padding)
        .style(move |_| container::Style {
            background: Some(theme.background.into()),
            text_color: Some(theme.text.into()),
            ..Default::default()
        });
        editor.into()
    }
}

impl<'a, Message> From<NoteEditor<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(editor: NoteEditor<'a, Message>) -> Self {
        component(editor)
    }
}
