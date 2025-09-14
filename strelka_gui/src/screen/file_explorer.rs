use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use iced::widget::{button, column, container, text};
use iced::{Element, Task};

use strelka_api::core::CoreAPI;
use strelka_api::message::{CoreAction, CoreMessage};
use strelka_core::Core;

use crate::BufferView;
use crate::message::Message;
use crate::screen::Screen;

#[derive(Debug, Clone)]
pub enum FileExplorerMessage {
    Initialized,
    Open(PathBuf),
}

#[derive(Debug, Clone)]
pub struct FileExplorer {
    path: PathBuf,
    files: Arc<RwLock<Vec<PathBuf>>>,
}

impl FileExplorer {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            files: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn init(&self, _core: &Arc<Core>) -> Task<FileExplorerMessage> {
        let path = self.path.clone();
        let files = self.files.clone();

        let init = async move {
            let mut collected = Vec::new();

            if let Ok(mut dir) = tokio::fs::read_dir(path).await {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    collected.push(entry.path());
                }
            }

            if let Ok(mut locked) = files.write() {
                *locked = collected;
            }

            FileExplorerMessage::Initialized
        };

        Task::perform(init, |msg| msg)
    }

    pub fn view(&self, _core: &Arc<Core>) -> Element<'_, FileExplorerMessage> {
        let files = self.files.read().unwrap();

        let mut col = column![];

        for path in files.iter() {
            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "<unknown>".into());

            col =
                col.push(button(text(filename)).on_press(FileExplorerMessage::Open(path.clone())));
        }

        container(col).into()
    }

    pub fn update(&mut self, core: &Arc<Core>, message: FileExplorerMessage) -> Task<Message> {
        match message {
            FileExplorerMessage::Initialized => Task::none(),
            FileExplorerMessage::Open(path) => {
                let core = core.clone();
                let task = async move {
                    let message = CoreMessage::OpenFile(path);
                    if let Some(CoreAction::DocumentOpened(buffer_id)) =
                        core.handle_command(message).await
                    {
                        let screen = BufferView::new(buffer_id);
                        let message = Message::SetScreen(Box::new(Screen::BufferView(screen)));
                        return message;
                    }
                    Message::None
                };

                Task::perform(task, |e| e)
            }
        }
    }
}
