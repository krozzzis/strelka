use core::{
    action::{ActionWrapper, DocumentAction, FileAction, PaneAction},
    document::DocumentHandler,
    pane::Pane,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use log::info;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct FileActor {
    receiver: Receiver<FileAction>,
    brocker_sender: Sender<ActionWrapper>,
}

impl FileActor {
    pub fn new(rx: Receiver<FileAction>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started FileActor's thread");
        while let Some(action) = self.receiver.recv().await {
            info!("Processing: {action:?}");
            match action {
                FileAction::PickFile => {
                    if let Ok((path, content)) = pick_file(None).await {
                        let handler = DocumentHandler {
                            text_content: content,
                            path: path.clone(),
                            filename: get_file_name(&path),
                            changed: false,
                        };

                        let (tx, mut rx) = channel(1);
                        let _ = self
                            .brocker_sender
                            .send(ActionWrapper::new(DocumentAction::Add(
                                Arc::new(handler),
                                Some(tx),
                            )))
                            .await;

                        if let Some(doc_id) = rx.recv().await {
                            let pane = Pane::Editor(doc_id);

                            // If opened pane is NewDocument, replace it with Editor pane
                            // otherwise add new one with Editor
                            let (tx, mut rx) = channel(1);
                            let _ = self
                                .brocker_sender
                                .send(ActionWrapper::new(PaneAction::GetOpen(tx)))
                                .await;
                            if let Some(Some(Pane::NewDocument)) = rx.recv().await {
                                let (tx, mut rx) = channel(1);
                                let _ = self
                                    .brocker_sender
                                    .send(ActionWrapper::new(PaneAction::GetOpenId(tx)))
                                    .await;

                                if let Some(Some(opened_id)) = rx.recv().await {
                                    let _ = self
                                        .brocker_sender
                                        .send(ActionWrapper::new(PaneAction::Replace(
                                            opened_id, pane,
                                        )))
                                        .await;
                                }
                            } else {
                                let (tx, mut rx) = channel(1);
                                let _ = self
                                    .brocker_sender
                                    .send(ActionWrapper::new(PaneAction::Add(pane, Some(tx))))
                                    .await;
                                if let Some(pane_id) = rx.recv().await {
                                    let _ = self
                                        .brocker_sender
                                        .send(ActionWrapper::new(PaneAction::Open(pane_id)))
                                        .await;
                                }
                            }
                        }
                    }
                }
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
