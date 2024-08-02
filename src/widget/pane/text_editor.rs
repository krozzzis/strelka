use iced::{
    widget::{
        text_editor::{Action, Content},
        Container,
    },
    Element,
};

use crate::theme::Theme;
use crate::widget::editor::NoteEditor;

pub fn text_editor_pane<'a, Message: 'a>(
    content: &'a Content,
    on_action: impl Fn(Action) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let editor = NoteEditor::new(content, Box::new(on_action)).theme(theme);

    Container::new(editor).into()
}
