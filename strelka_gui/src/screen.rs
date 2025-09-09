use std::sync::Arc;

use iced::Element;
use iced::widget::{text, column, container};
use strelka_core::{Core, BufferId};


pub trait Screen<'a, Message> {
    fn view(&self, core: Arc<Core>) -> Element<'a, Message> {
        text("None").into()
    }
}

pub struct BufferView {
    pub buffer_id: BufferId,
}

impl BufferView {
    pub fn new(buffer_id: BufferId) -> Self {
        Self {
            buffer_id,
        }
    }
}

impl<'a, T: 'a> Screen<'a, T> for BufferView {
    fn view(&self, core: Arc<Core>) -> Element<'a, T> {
        if let Some(buffer) = core.buffers.get(&self.buffer_id) {
            container(
                column![text(buffer.key()), text(buffer.value().content.clone())],
            ).into()
        } else {
            text("Not Found").into()
        }
    }
}
