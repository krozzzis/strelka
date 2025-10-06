mod buffer_list;
mod buffer_view;
mod file_explorer;
mod screen_manager;

pub use buffer_list::*;
pub use buffer_view::*;
pub use file_explorer::*;
pub use screen_manager::*;

use std::sync::Arc;

use iced::{Element, Task};

use crate::message::Message;
use strelka_core::Core;

#[derive(Debug, Clone)]
pub enum ScreenMessageRaw {
    BufferView(BufferViewMessage),
    BufferList(BufferListMessage),
    FileExplorer(FileExplorerMessage),
}

#[derive(Debug, Clone)]
pub struct ScreenMessage {
    pub screen_id: ScreenId,
    pub raw: ScreenMessageRaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenKind {
    BufferView,
    BufferList,
    FileExplorer,
    Blank,
}

#[derive(Debug, Clone)]
pub enum Screen {
    BufferView(BufferView),
    BufferList(BufferList),
    FileExplorer(FileExplorer),
    Blank,
}

impl Screen {
    pub fn view(&self, screen_id: ScreenId, core: &Arc<Core>) -> Element<'_, Message> {
        match self {
            Screen::BufferView(state) => state.view(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::BufferView(e),
                })
            }),
            Screen::BufferList(state) => state.view(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::BufferList(e),
                })
            }),
            Screen::FileExplorer(state) => state.view(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::FileExplorer(e),
                })
            }),
            Screen::Blank => iced::widget::center(iced::widget::text("Nothing here..."))
                .width(iced::Fill)
                .height(iced::Fill)
                .into(),
        }
    }

    pub fn init(&self, screen_id: ScreenId, core: &Arc<Core>) -> Task<Message> {
        match self {
            Screen::BufferView(state) => state.init(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::BufferView(e),
                })
            }),
            Screen::BufferList(state) => state.init(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::BufferList(e),
                })
            }),
            Screen::FileExplorer(state) => state.init(core).map(move |e| {
                Message::Screen(ScreenMessage {
                    screen_id,
                    raw: ScreenMessageRaw::FileExplorer(e),
                })
            }),
            Screen::Blank => Task::none(),
        }
    }
}
