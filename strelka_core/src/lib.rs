mod gui_service;

pub use gui_service::MessageBasedGuiService;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::fs;

use strelka_api::BufferId;
use strelka_api::core::CoreAPI;
use strelka_api::message::{CoreAction, CoreMessage};

#[derive(Debug)]
pub struct Core {
    pub buffers: Arc<DashMap<BufferId, Buffer>>,
    next_buffer_id: AtomicU64,
}

impl Core {
    pub fn new() -> Self {
        let buffers = Arc::new(DashMap::new());

        Self {
            buffers,
            next_buffer_id: AtomicU64::new(1),
        }
    }
}

#[async_trait]
impl CoreAPI for Core {
    async fn handle_command(&self, cmd: CoreMessage) -> Option<CoreAction> {
        println!("Command: {cmd:?}");

        match cmd {
            CoreMessage::InsertText(buffer_id, text) => {
                let buffer = self.buffers.get_mut(&buffer_id);
                if let Some(mut buffer) = buffer {
                    let len = buffer.content.len();
                    buffer.content.insert_str(len, text.as_str());
                    return Some(CoreAction::None);
                }
            }
            CoreMessage::OpenFile(path) => {
                if let Ok(content) = fs::read_to_string(&path).await {
                    let buffer_id = self.next_buffer_id.fetch_add(1, Ordering::SeqCst);
                    let buffer = Buffer { content };
                    self.buffers.insert(buffer_id, buffer);
                    return Some(CoreAction::DocumentOpened(buffer_id));
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub content: String,
}
