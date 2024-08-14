use iced::{
    border::Radius,
    widget::{
        center, component, container,
        text_editor::{self, Content},
        Component, Container, TextEditor,
    },
    Border, Element, Length, Pixels,
};

use theming::{self, theme, Theme};

/// Text editor widget
pub struct NoteEditor<'a, Message> {
    content: &'a Content,
    on_action: Box<dyn Fn(text_editor::Action) -> Message>,
}

impl<'a, Message> NoteEditor<'a, Message> {
    pub fn new<F>(content: &'a Content, on_action: F) -> Self
    where
        F: 'static + Fn(text_editor::Action) -> Message,
    {
        Self {
            content,
            on_action: Box::new(on_action),
        }
    }
}

impl<'a, Message> Component<Message, Theme> for NoteEditor<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme> {
        let editor = Container::new(center(
            Container::new(
                TextEditor::new(self.content)
                    .on_action(&self.on_action)
                    .height(Length::Fill)
                    .size(16.0),
            )
            .padding(theme!(editor.padding))
            .max_width(Pixels::from(700.0))
            .style(|theme: &Theme| container::Style {
                border: Border {
                    radius: Radius::from(theme.editor.radius),
                    ..Default::default()
                },
                ..theming::iced::container::background2(theme)
            }),
        ))
        .padding(theme!(editor.padding))
        .style(|theme: &Theme| container::Style {
            background: Some(theme.editor.background.into()),
            text_color: Some(theme.editor.text.into()),
            ..Default::default()
        });
        editor.into()
    }
}

impl<'a, Message> From<NoteEditor<'a, Message>> for Element<'a, Message, Theme>
where
    Message: 'a,
{
    fn from(editor: NoteEditor<'a, Message>) -> Self {
        component(editor)
    }
}
