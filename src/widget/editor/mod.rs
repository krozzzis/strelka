use iced::{
    border::Radius,
    widget::{
        center, component,
        text_editor::{self, Content},
        Component, Container, TextEditor,
    },
    Background, Border, Color, Element, Length, Pixels, Theme,
};

use crate::styles;

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

impl<'a, Message> Component<Message> for NoteEditor<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let editor = Container::new(center(
            Container::new(
                TextEditor::new(self.content)
                    .on_action(&self.on_action)
                    .height(Length::Fill)
                    .font(styles::INTER_REGULAR_FONT)
                    .size(16.0)
                    .style(|theme: &Theme, _status| text_editor::Style {
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::new(0.0),
                        },
                        background: Background::Color(theme.palette().background),
                        icon: theme.palette().text,
                        placeholder: theme.palette().primary,
                        value: theme.palette().danger,
                        selection: theme.palette().primary,
                    }),
            )
            .padding(32.0)
            .max_width(Pixels::from(700.0)),
        ));
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
