use std::path::{Path, PathBuf};

use iced::Element;
use iced::{
    widget::{
        column,
        text_editor::{Action, Content},
        Container,
    },
    Length,
};

use crate::{get_file_name, widget::editor::NoteEditor};
use crate::{theme::Theme, widget::tabs::tab_bar};

pub fn text_editor_pane<'a, Message: 'a + Clone>(
    content: &'a Content,
    on_action: impl Fn(Action) -> Message + 'static,
    file_name: String,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let editor = Container::new(NoteEditor::new(content, Box::new(on_action)).theme(theme))
        .height(Length::Fill);
    let tabs = tab_bar(Box::new([file_name]), theme);

    Container::new(column![tabs, editor]).into()
}
