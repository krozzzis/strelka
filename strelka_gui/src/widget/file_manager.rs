use std::path::PathBuf;

use iced::{
    Element, Task,
    widget::{button, column, scrollable, text},
};

#[derive(Debug, Clone)]
pub enum Action {
    UpdateFiles(Vec<PathBuf>),
}

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),
    Open(PathBuf),
}

#[derive(Debug, Clone)]
pub struct State {
    directory: PathBuf,
    files: Vec<PathBuf>,
}

impl State {
    pub fn new(directory: impl Into<PathBuf>) -> (Self, Task<Message>) {
        let directory = directory.into();
        (
            Self {
                directory: directory.clone(),
                files: Vec::new(),
            },
            Self::init(directory),
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = self
            .files
            .iter()
            .map(|path| {
                button(text(path.file_name().unwrap_or_default().to_string_lossy()))
                    .on_press(Message::Open(path.clone()))
                    .into()
            })
            .collect::<Vec<_>>();

        scrollable(column(content)).into()
    }

    pub fn take_action(&mut self, action: Action) -> Task<Message> {
        match action {
            Action::UpdateFiles(files) => {
                self.files = files;
            }
        }

        Task::none()
    }

    fn init(directory: PathBuf) -> Task<Message> {
        Task::perform(
            async move {
                use tokio::fs;
                use tokio_stream::StreamExt;

                let mut files = Vec::new();

                if let Ok(mut entries) = fs::read_dir(&directory).await {
                    while let Some(entry) = entries.next_entry().await.transpose() {
                        if let Ok(entry) = entry {
                            files.push(entry.path());
                        }
                    }
                }

                files
            },
            |files| Message::Action(Action::UpdateFiles(files)),
        )
    }
}
