use state::State;

use iced::widget::{column, container, text, Container};
use iced::{Element, Length};

use crate::file_explorer;
use theming::Theme;

pub fn file_explorer_pane<'a>(
    state: &State,
    file_explorer: &'a file_explorer::State,
) -> Element<'a, file_explorer::Message, Theme> {
    let explorer = file_explorer.view();
    let pane = Container::new(
        column![
            container(text(state.working_directory.to_string_lossy().to_string())).padding(4.0),
            explorer
        ]
        .spacing(8.0),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|theme: &Theme| container::Style {
        background: Some(theme.file_explorer.background.into()),
        text_color: Some(theme.file_explorer.text.into()),
        ..theming::iced::container::background(theme)
    });

    pane.into()
}
