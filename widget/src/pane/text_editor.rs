use core::document::{DocumentId, DocumentStore};

use iced::{
    widget::{
        center, container,
        text_editor::{Action, Content},
        Space,
    },
    Element, Length,
};
use theming::Theme;

use crate::editor::NoteEditor;

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(Action),
}

pub fn text_editor(
    id: DocumentId,
    documents: &DocumentStore<Content>,
) -> Element<'_, Message, Theme> {
    if let Some(handler) = documents.get(&id) {
        let editor = container(center(
            container(NoteEditor::new(
                &handler.text_content,
                Message::EditorAction,
            ))
            .width(600.0),
        ));
        editor.into()
    } else {
        container(Space::new(Length::Fill, Length::Fill)).into()
    }
}
