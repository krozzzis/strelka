use iced::{
    border::Radius,
    widget::{
        center, component, container,
        text_editor::{self, Content},
        Component, Container, TextEditor,
    },
    Border, Color, Element, Length, Pixels,
};

use crate::{styles, theming::Theme};

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
        let theme = self.theme.cloned().unwrap_or(Theme::default());
        let editor = Container::new(center(
            Container::new(
                TextEditor::new(self.content)
                    .on_action(&self.on_action)
                    .height(Length::Fill)
                    .font(styles::INTER_REGULAR_FONT)
                    .size(16.0)
                    .style(move |_, _status| text_editor::Style {
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::new(0.0),
                        },
                        background: theme.background.into(),
                        icon: theme.selected,
                        placeholder: theme.subtext,
                        value: theme.selected,
                        selection: theme.primary,
                    }),
            )
            .padding(32.0)
            .max_width(Pixels::from(700.0))
            .style(move |_| {
                let theme = self.theme.cloned().unwrap_or(Theme::default());
                container::Style {
                    background: Some(theme.background.into()),
                    text_color: Some(theme.text),
                    ..Default::default()
                }
            }),
        ))
        .style(move |_| {
            let theme = self.theme.cloned().unwrap_or(Theme::default());
            container::Style {
                background: Some(theme.background.into()),
                text_color: Some(theme.text),
                ..Default::default()
            }
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
