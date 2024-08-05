use std::path::PathBuf;

use iced::widget::{container, Container};
use iced::{Element, Length};

use crate::{theming::Theme, widget::file_explorer::FileExplorer};

pub fn file_explorer_pane<'a, Message: 'a + Clone>(
    content: Option<&'a Vec<PathBuf>>,
    selected_file: Option<PathBuf>,
    open_file: impl Fn(PathBuf) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let explorer = Container::new(
        FileExplorer::with_content_maybe(content.map(|x| x.as_slice()))
            .file_click(open_file)
            .select_file_maybe(selected_file)
            .theme(theme),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(theme.theme.file_explorer.background.into()),
        text_color: Some(theme.theme.file_explorer.text.into()),
        ..Default::default()
    });

    explorer.into()
}
