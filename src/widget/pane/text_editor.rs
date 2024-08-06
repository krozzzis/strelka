use std::{collections::HashMap, sync::Arc};

use iced::{widget::center, Element};
use iced::{
    widget::{column, text_editor::Action, Container},
    Length,
};

use crate::{
    theming::Theme,
    widget::{button::text_button, containers::background, tabs::tab_bar},
};
use crate::{widget::editor::NoteEditor, DocumentHandler, DocumentId};

pub fn text_editor_pane<'a, Message: 'a + Clone>(
    documents: &'a HashMap<DocumentId, DocumentHandler>,
    current_document: DocumentId,
    on_action: impl Fn(Action, DocumentId) -> Message + 'static,
    open_document: impl Fn(DocumentId) -> Message + 'static,
    close_document: impl Fn(DocumentId) -> Message + 'static,
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
        background(
            center(text_button("Open file Ctrl+O", theme).on_press_maybe(pick_file)),
            theme,
        )
    };

    let tabs = tab_bar(
        documents
            .iter()
            .map(|(id, handler)| {
                (
                    Arc::new(format!(
                        "{} {}",
                        handler.filename,
                        if handler.changed { "*" } else { "" }
                    )),
                    open_document(*id),
                    Some(close_document(*id)),
                )
            })
            .collect(),
        None,
        theme,
    );

    Container::new(column![tabs, editor]).into()
}
