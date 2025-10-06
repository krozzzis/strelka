use std::sync::Arc;

use iced::widget::{button, column, container, text};
use iced::{Element, Task};

use strelka_core::Core;

use crate::message::Message;
use crate::screen::BufferView;
use crate::screen::Screen;
use strelka_api::BufferId;

#[derive(Debug, Clone)]
pub enum BufferListMessage {
    Open(BufferId),
}

#[derive(Debug, Clone)]
pub struct BufferList {}

impl BufferList {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self, _core: &Arc<Core>) -> Task<BufferListMessage> {
        Task::none()
    }

    pub fn view(&self, core: &Arc<Core>) -> Element<'_, BufferListMessage> {
        let buffers: Vec<BufferId> = core.buffers.iter().map(|entry| *entry.key()).collect();

        let mut col = column![];

        for id in buffers.iter() {
            col =
                col.push(button(text(format!("ID: {id}"))).on_press(BufferListMessage::Open(*id)));
        }

        container(col).into()
    }

    pub fn update(&mut self, _core: &Arc<Core>, message: BufferListMessage) -> Task<Message> {
        match message {
            BufferListMessage::Open(buffer_id) => {
                let task = async move {
                    let screen = BufferView::new(buffer_id);
                    //Message::SetScreen(Box::new(Screen::BufferView(screen)))
                    Message::None
                };
                Task::perform(task, |e| e)
            }
        }
    }
}

impl From<BufferList> for Screen {
    fn from(screen: BufferList) -> Screen {
        Screen::BufferList(screen)
    }
}
