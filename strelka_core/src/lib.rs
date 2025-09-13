use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;
use tokio::fs;

pub type BufferId = u64;

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

    pub async fn handle_command(&self, cmd: CoreCommand) -> Option<CoreEvent> {
        println!("Command: {cmd:?}");

        match cmd.action {
            CoreAction::InsertText(buffer_id, text) => {
                let buffer = self.buffers.get_mut(&buffer_id);
                if let Some(mut buffer) = buffer {
                    let len = buffer.content.len();
                    buffer.content.insert_str(len, text.as_str());
                    return Some(CoreEvent::None);
                }
            }
            CoreAction::OpenFile(path) => {
                if let Ok(content) = fs::read_to_string(&path).await {
                    let buffer_id = self.next_buffer_id.fetch_add(1, Ordering::SeqCst);
                    let buffer = Buffer { content };
                    self.buffers.insert(buffer_id, buffer);
                    return Some(CoreEvent::DocumentOpened(buffer_id));
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub enum CoreEvent {
    DocumentOpened(BufferId),
    None,
}

#[derive(Debug, Clone)]
pub struct CoreCommand {
    pub action: CoreAction,
}

impl CoreCommand {
    pub fn new(action: CoreAction) -> Self {
        Self { action }
    }
}

#[derive(Debug, Clone)]
pub enum CoreAction {
    InsertText(BufferId, Arc<String>),
    OpenFile(PathBuf),
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub content: String,
}
