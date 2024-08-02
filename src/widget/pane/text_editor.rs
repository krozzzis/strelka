use std::collections::HashMap;

use iced::{
    widget::{button, center},
    Element,
};
use iced::{
    widget::{column, text_editor::Action, Container},
    Length,
};

use crate::{theme::Theme, widget::tabs::tab_bar};
use crate::{widget::editor::NoteEditor, DocumentHandler, DocumentId};

pub fn text_editor_pane<'a, Message: 'a + Clone>(
    documents: &'a HashMap<DocumentId, DocumentHandler>,
    current_document: DocumentId,
    on_action: impl Fn(Action, DocumentId) -> Message + 'static,
    open_document: impl Fn(DocumentId) -> Message + 'static,
    pick_file: Option<Message>,
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
        Container::new(center(
            button("Open file Ctrl+O")
                .on_press_maybe(pick_file)
                .style(theme.transparent_button()),
        ))
    };

    let tabs = tab_bar(
        documents
            .iter()
            .map(|(id, handler)| (handler.filename.clone(), open_document(*id)))
            .collect(),
        None,
        theme,
    );

    Container::new(column![tabs, editor]).into()
}
