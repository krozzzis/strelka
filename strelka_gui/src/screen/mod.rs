mod buffer_view;
mod file_explorer;

pub use buffer_view::*;
pub use file_explorer::*;

use std::sync::Arc;

use iced::{Element, Task};

use crate::message::Message;
use strelka_core::Core;

#[derive(Debug)]
pub enum ScreenMessage {
    BufferView(BufferViewMessage),
    FileExplorer(FileExplorerMessage),
}

#[derive(Debug)]
pub enum Screen {
    BufferView(BufferView),
    FileExplorer(FileExplorer),
}

impl Screen {
    pub fn view(&self, core: &Arc<Core>) -> Element<'_, Message> {
        match self {
            Screen::BufferView(state) => state
                .view(core)
                .map(|e| Message::Screen(ScreenMessage::BufferView(e))),
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
            Screen::FileExplorer(state) => state
                .init(core)
                .map(|e| Message::Screen(ScreenMessage::FileExplorer(e))),
        }
    }
}
