use action::{Action, FileAction};
use std::path::{Path, PathBuf};

use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct FileActor {
    receiver: Receiver<Action>,
    brocker_sender: Sender<Action>,
}

impl FileActor {
    pub fn new(rx: Receiver<Action>, brocker_tx: Sender<Action>) -> Self {
        Self {
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started FileActor");
        while let Some(generic_action) = self.receiver.recv().await {
            info!("FileActor. Processing: {generic_action:?}");
            let action = if let Ok(x) = generic_action.content.downcast() {
                x
            } else {
                warn!("FileActor. Dropping processing action because incorrect type");
                continue;
            };
            match *action {
                FileAction::PickFile => todo!(),
                FileAction::OpenFileCurrentTab(_path) => {
                    todo!()
                }
                FileAction::OpenFileForceCurrentTab(_path) => {
                    todo!()
                }
                FileAction::OpenFileNewTab(_path) => {
                    todo!()
                }
            }
        }
    }
}

pub async fn pick_file(directory: Option<PathBuf>) -> Result<(PathBuf, String), ()> {
    let handler = if let Some(dir) = directory {
        rfd::AsyncFileDialog::new().set_directory(dir)
    } else {
        rfd::AsyncFileDialog::new()
    }
    .pick_file()
    .await;

    if let Some(path) = handler {
        let content = open_file(path.path()).await.map_err(|_| ())?;
        Ok(content)
    } else {
        Err(())
    }
}

pub async fn open_file(path: impl AsRef<Path>) -> Result<(PathBuf, String), ()> {
    let path = path.as_ref();
    let content = tokio::fs::read_to_string(path).await.map_err(|_| ())?;
    Ok((path.to_owned(), content))
}

pub fn get_file_name(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_owned()
}
