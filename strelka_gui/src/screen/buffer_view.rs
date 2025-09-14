use std::sync::Arc;

use iced::widget::text_editor;
use iced::{Element, Length, Task};

use strelka_api::BufferId;
use strelka_core::Core;

use crate::message::Message;

#[derive(Debug, Clone)]
pub enum BufferViewMessage {
    TextEditorAction(text_editor::Action),
    None,
}

#[derive(Debug, Clone)]
pub struct BufferView {
    pub buffer_id: BufferId,
    content: text_editor::Content,
}

impl BufferView {
    pub fn new(buffer_id: BufferId) -> Self {
        Self {
            buffer_id,
            content: text_editor::Content::new(),
        }
    }

    pub fn init(&self, core: &Arc<Core>) -> Task<BufferViewMessage> {
        let id = self.buffer_id;
        let core = core.clone();
        let init = async move {
            if let Some(buffer) = core.buffers.get(&id) {
                let content = buffer.content.clone();
                let action = text_editor::Action::Edit(text_editor::Edit::Paste(Arc::new(content)));
                BufferViewMessage::TextEditorAction(action)
            } else {
                BufferViewMessage::None
            }
        };

        Task::perform(init, |e| e)
    }

    pub fn view(&self, _core: &Arc<Core>) -> Element<'_, BufferViewMessage> {
        text_editor(&self.content)
            .placeholder("Type something here...")
            .on_action(BufferViewMessage::TextEditorAction)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, _core: &Arc<Core>, message: BufferViewMessage) -> Task<Message> {
        match message {
            BufferViewMessage::TextEditorAction(e) => {
                self.content.perform(e);
            }
            BufferViewMessage::None => {}
        }
        Task::none()
    }
}
