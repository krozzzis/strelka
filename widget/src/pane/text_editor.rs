use core::{document::DocumentId, value::Value};
use state::State;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use iced::{
    border::Radius,
    widget::{center, column, container, text, text_editor::Action, Row, Space},
    Alignment, Border, Element, Length,
};
use theming::{theme, Theme};

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

pub fn text_editor(id: DocumentId, state: &State) -> Element<'_, Message, Theme> {
    let title: Element<'_, Message, Theme> = if let Some(handler) = state.documents.get(&id) {
        let working_directory =
            if let Some(Value::String(path)) = state.config.get("system", "workdir") {
                PathBuf::from_str(&path).unwrap_or_default()
            } else {
                PathBuf::new()
            };

        let mut folders = get_directories_between(
            {
                if handler.path.starts_with(&working_directory) {
                    working_directory.as_path()
                } else {
                    Path::new("/")
                }
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
        let width = theme!(editor.width);

        let editor = container(center(column![
            container(title)
                .height(theme!(editor.padding))
                .width(width)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container(NoteEditor::new(
                &handler.text_content,
                Message::EditorAction,
            ))
            .width(width)
            .padding(theme!(editor.padding))
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
