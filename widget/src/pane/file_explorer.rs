use iced::widget::{container, Container};
use iced::{Element, Length};

use crate::file_explorer::{self, State};
use theming::Theme;

pub fn file_explorer_pane(state: &State) -> Element<file_explorer::Message, Theme> {
    let explorer = Container::new(state.view())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.file_explorer.background.into()),
            text_color: Some(theme.file_explorer.text.into()),
            ..theming::iced::container::background(theme)
        });

    explorer.into()
}
