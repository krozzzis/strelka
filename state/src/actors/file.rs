use action::{Action, DocumentAction, FileAction, IntoAction, PaneAction};
use core::{document::DocumentHandler, pane::Pane};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use log::{info, warn};
use tokio::sync::mpsc::{channel, Receiver, Sender};

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
                FileAction::PickFile => {
                    if let Ok((path, content)) = pick_file(None).await {
                        let handler = DocumentHandler {
                            text_content: content,
                            path: path.clone(),
                            filename: get_file_name(&path),
                            changed: false,
                        };

                        let (action, rx) =
                            DocumentAction::Add(Arc::new(handler)).into_returnable_action();
                        let _ = self.brocker_sender.send(action).await;

                        if let Some(rx) = rx {
                            if let Ok(action::ActionResult::Value(doc_id)) = rx.await {
                                if let Ok(doc_id) = doc_id.downcast() {
                                    let pane = Pane::Editor(*doc_id);

                                    // If opened pane is NewDocument, replace it with Editor pane
                                    // otherwise add new one with Editor
                                    let (tx, mut rx) = channel(1);
                                    let _ = self
                                        .brocker_sender
                                        .send(PaneAction::GetOpen(tx).into_action())
                                        .await;
                                    if let Some(Some(Pane::NewDocument)) = rx.recv().await {
                                        let (tx, mut rx) = channel(1);
                                        let _ = self
                                            .brocker_sender
                                            .send(PaneAction::GetOpenId(tx).into_action())
                                            .await;

                                        if let Some(Some(opened_id)) = rx.recv().await {
                                            let _ = self
                                                .brocker_sender
                                                .send(
                                                    PaneAction::Replace(opened_id, pane)
                                                        .into_action(),
                                                )
                                                .await;
                                        }
                                    } else {
                                        let (tx, mut rx) = channel(1);
                                        let _ = self
                                            .brocker_sender
                                            .send(PaneAction::Add(pane, Some(tx)).into_action())
                                            .await;
                                        if let Some(pane_id) = rx.recv().await {
                                            let _ = self
                                                .brocker_sender
                                                .send(PaneAction::Open(pane_id).into_action())
                                                .await;
                                        }
                                    }
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
