use std::collections::HashMap;

use iced::{widget::center, Element};
use iced::{
    widget::{column, text, text_editor::Action, Container},
    Length,
};

use crate::{theme::Theme, widget::tabs::tab_bar};
use crate::{widget::editor::NoteEditor, DocumentHandler, DocumentId};

pub fn text_editor_pane<'a, Message: 'a + Clone>(
    documents: &'a HashMap<DocumentId, DocumentHandler>,
    current_document: DocumentId,
    on_action: impl Fn(Action, DocumentId) -> Message + 'static,
    open_document: impl Fn(DocumentId) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let editor = if let Some(handler) = documents.get(&current_document) {
        Container::new(
            NoteEditor::new(
                &handler.text_content,
                Box::new(move |action| on_action(action, current_document)),
            )
            .theme(theme),
        )
        .height(Length::Fill)
    } else {
        Container::new(center(text("New file")))
    };

    let tabs = tab_bar(
        documents
            .iter()
            .map(|(id, handler)| (handler.filename.clone(), open_document(*id)))
            .collect(),
        theme,
    );

    Container::new(column![tabs, editor]).into()
}
