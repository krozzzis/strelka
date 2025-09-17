mod buffer_list;
mod buffer_view;
mod file_explorer;

pub use buffer_list::*;
pub use buffer_view::*;
pub use file_explorer::*;

use std::sync::Arc;

use iced::{Element, Task};

use crate::message::Message;
use strelka_core::Core;

#[derive(Debug, Clone)]
pub enum ScreenMessage {
    BufferView(BufferViewMessage),
    BufferList(BufferListMessage),
    FileExplorer(FileExplorerMessage),
}

#[derive(Debug, Clone)]
pub enum Screen {
    BufferView(BufferView),
    BufferList(BufferList),
    FileExplorer(FileExplorer),
}

impl Screen {
    pub fn view(&self, core: &Arc<Core>) -> Element<'_, Message> {
        match self {
            Screen::BufferView(state) => state
                .view(core)
                .map(|e| Message::Screen(ScreenMessage::BufferView(e))),
            Screen::BufferList(state) => state
                .view(core)
                .map(|e| Message::Screen(ScreenMessage::BufferList(e))),
            Screen::FileExplorer(state) => state
                .view(core)
                .map(|e| Message::Screen(ScreenMessage::FileExplorer(e))),
        }
    }

    pub fn init(&self, core: &Arc<Core>) -> Task<Message> {
        match self {
            Screen::BufferView(state) => state
                .init(core)
                .map(|e| Message::Screen(ScreenMessage::BufferView(e))),
            Screen::BufferList(state) => state
                .init(core)
                .map(|e| Message::Screen(ScreenMessage::BufferList(e))),
            Screen::FileExplorer(state) => state
                .init(core)
                .map(|e| Message::Screen(ScreenMessage::FileExplorer(e))),
        }
    }
}
