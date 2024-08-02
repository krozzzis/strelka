use std::path::PathBuf;

use iced::widget::{container, Container};
use iced::{Element, Length};

use crate::{theme::Theme, widget::file_explorer::FileExplorer};

pub fn file_explorer_pane<'a, Message: 'a + Clone>(
    content: Option<&'a Vec<PathBuf>>,
    open_file: impl Fn(PathBuf) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let explorer = Container::new(
        FileExplorer::with_content_maybe(content.map(|x| x.as_slice()))
            .file_click(open_file)
            .theme(theme),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| container::Style {
        background: Some(theme.background.into()),
        ..Default::default()
    });

    explorer.into()
}
