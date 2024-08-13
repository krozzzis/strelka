use std::{path::PathBuf, sync::Arc};

use iced::widget::{container, Container};
use iced::{Element, Length};

use crate::widget::file_explorer::{FileExplorer, State};
use theming::Theme;

pub fn file_explorer_pane<'a, Message: 'a + Clone>(
    dir: Arc<PathBuf>,
    state: &'a State,
    selected_file: Option<PathBuf>,
    open_file: impl Fn(PathBuf) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let explorer = Container::new(
        FileExplorer::new(dir, state)
            .file_click(open_file)
            .select_file_maybe(selected_file)
            .theme(theme),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(theme.file_explorer.background.into()),
        text_color: Some(theme.file_explorer.text.into()),
        ..Default::default()
    });

    explorer.into()
}
