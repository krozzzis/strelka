use core::{document::DocumentId, State};

use iced::{
    border::Radius,
    widget::{
        center, container,
        text_editor::{Action, Content},
        Space,
    },
    Border, Element, Length, Padding,
};
use theming::Theme;

use crate::editor::NoteEditor;

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(Action),
}

pub fn text_editor(id: DocumentId, state: State<'_, Content>) -> Element<'_, Message, Theme> {
    if let Some(handler) = state.documents.get(&id) {
        let editor = container(
            center(
                container(NoteEditor::new(
                    &handler.text_content,
                    Message::EditorAction,
                ))
                .padding(50.0)
                .width(600.0)
                .style(|theme: &Theme| container::Style {
                    text_color: Some(theme.editor.text.into()),
                    background: Some(theme.editor.background2.into()),
                    border: Border::default().rounded(Radius {
                        top_left: theme.editor.radius,
                        top_right: theme.editor.radius,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .padding(Padding::from(0.0).top(50.0)),
        );
        editor.into()
    } else {
        container(Space::new(Length::Fill, Length::Fill)).into()
    }
}
