use core::{document::DocumentId, State};
use std::path::{Path, PathBuf};

use iced::{
    border::Radius,
    widget::{
        center, column, container, text,
        text_editor::{Action, Content},
        Row, Space,
    },
    Alignment, Border, Element, Length,
};
use theming::Theme;

use crate::{editor::NoteEditor, util::filename};

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(Action),
}

fn get_directories_between<'a>(base: &'a Path, target: &'a Path) -> Vec<String> {
    if let Ok(relative_path) = target.strip_prefix(base) {
        relative_path
            .components()
            .filter_map(|component| match component {
                std::path::Component::Normal(os_str) => os_str.to_str().map(String::from),
                _ => None,
            })
            .collect()
    } else {
        Vec::new()
    }
}

pub fn text_editor(id: DocumentId, state: State<'_, Content>) -> Element<'_, Message, Theme> {
    let title: Element<'_, Message, Theme> = if let Some(handler) = state.documents.get(&id) {
        let mut folders = get_directories_between(
            if handler.path.starts_with(&state.working_directory) {
                &state.working_directory
            } else {
                Path::new("/")
            },
            &handler.path,
        );
        if let Some(last) = folders.last_mut() {
            *last = filename(PathBuf::from(last.clone())).unwrap_or_default();
        }

        let mut items = Vec::new();
        for (i, folder) in folders.iter().enumerate() {
            let item = text(folder.clone()).into();
            items.push(item);
            if i != folders.len() - 1 {
                items.push(text("/").into());
            }
        }
        Row::with_children(items).spacing(4.0).into()
    } else {
        text("").into()
    };

    if let Some(handler) = state.documents.get(&id) {
        let editor = container(center(column![
            container(title)
                .height(50.0)
                .width(600.0)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container(NoteEditor::new(
                &handler.text_content,
                Message::EditorAction,
            ))
            .width(600.0)
            .padding(50.0)
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
        ]));
        editor.into()
    } else {
        container(Space::new(Length::Fill, Length::Fill)).into()
    }
}
