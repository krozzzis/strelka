use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::PathBuf;

use crossbeam_channel::{Sender, Receiver};
use dashmap::DashMap;
use tokio::fs;

pub type BufferId = u64;
 
#[derive(Debug)]
pub struct Core {
    pub buffers: Arc<DashMap<BufferId, Buffer>>,
    command_rx: Receiver<CoreCommand>,
    event_tx: Sender<CoreEvent>,
    next_buffer_id: AtomicU64,
}

impl Core {
    pub fn new(command_rx: Receiver<CoreCommand>, event_tx: Sender<CoreEvent>) -> Self {
        let buffers = Arc::new(DashMap::new());

        Self {
            buffers,
            command_rx,
            event_tx,
            next_buffer_id: AtomicU64::new(1), // Start from 3 since we already have buffers 1 and 2
        }
    }
    pub async fn run(&mut self) {
        // Главный цикл Core
        while let Ok(cmd) = self.command_rx.recv() {
            self.handle_command(cmd).await;
        }
    }

    pub async fn handle_command(&self, cmd: CoreCommand) {
        println!("Command: {cmd:?}");
        match cmd.action {
            CoreAction::InsertText(buffer_id, text) => {
                let buffer = self.buffers.get_mut(&buffer_id);
                if let Some(mut buffer) = buffer {
                    let len = buffer.content.len();
                    buffer.content.insert_str( len, text.as_str());
                    let _ = self.event_tx.send(CoreEvent::None);
                }
            },
            CoreAction::OpenFile(path) => {
                if let Ok(content) = fs::read_to_string(&path).await {
                    let buffer_id = self.next_buffer_id.fetch_add(1, Ordering::SeqCst);
                    let buffer = Buffer { content };
                    self.buffers.insert(buffer_id, buffer);
                    let _ = self.event_tx.send(CoreEvent::DocumentOpened(buffer_id));
                }
            },
        }
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

#[derive(Debug, Clone)]
pub enum CoreAction {
    InsertText(BufferId, Arc<String>),
    OpenFile(PathBuf),
}


#[derive(Debug, Clone)]
pub struct Buffer {
    pub content: String,
}