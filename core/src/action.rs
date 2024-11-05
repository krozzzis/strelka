use std::{path::PathBuf, sync::Arc};

use tokio::sync::{broadcast, mpsc};

use crate::{
    document::{DocumentHandler, DocumentId},
    pane::{Pane, PaneId, VisiblePaneModel},
    ThemeId,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub destination: String,
    pub kind: String,
    pub payload: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PaneAction {
    Close(PaneId),
    Open(PaneId),
    Add(Pane, Option<mpsc::Sender<PaneId>>),
    Replace(PaneId, Pane),
    GetOpen(mpsc::Sender<Option<Pane>>),
    GetOpenId(mpsc::Sender<Option<PaneId>>),
    GetModel(mpsc::Sender<Option<VisiblePaneModel>>),
}

#[derive(Debug, Clone)]
pub enum FileAction {
    PickFile,
    OpenFileCurrentTab(PathBuf),
    OpenFileForceCurrentTab(PathBuf),
    OpenFileNewTab(PathBuf),
}

#[derive(Debug, Clone)]
pub enum DocumentAction {
    Add(
        Arc<DocumentHandler<String>>,
        Option<mpsc::Sender<DocumentId>>,
    ),
    Open(DocumentId),
    Save(DocumentId),
    Remove(DocumentId),
}

#[derive(Debug, Clone)]
pub enum DocumentActionResponse {
    DocumentAdded(DocumentId),
}

#[derive(Debug, Clone)]
pub enum ThemeAction {
    MakeIndex,
    SetTheme(ThemeId),
}

#[derive(Debug, Clone)]
pub enum Action {
    File(FileAction),
    Pane(PaneAction),
    Document(DocumentAction),
    Theme(ThemeAction),
    Message(Message),
}

impl From<FileAction> for Action {
    fn from(value: FileAction) -> Self {
        Self::File(value)
    }
}

impl From<PaneAction> for Action {
    fn from(value: PaneAction) -> Self {
        Self::Pane(value)
    }
}

impl From<DocumentAction> for Action {
    fn from(value: DocumentAction) -> Self {
        Self::Document(value)
    }
}

impl From<ThemeAction> for Action {
    fn from(value: ThemeAction) -> Self {
        Self::Theme(value)
    }
}

impl From<Message> for Action {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}

#[derive(Debug)]
pub struct ActionWrapper {
    pub action: Action,
    pub completition_tx: Option<broadcast::Sender<ActionResult>>,
}

impl ActionWrapper {
    pub fn new(action: impl Into<Action>) -> Self {
        Self {
            action: action.into(),
            completition_tx: None,
        }
    }

    pub fn notify(mut self, sender: broadcast::Sender<ActionResult>) -> Self {
        self.completition_tx = Some(sender);
        self
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn try_notify_complete(&self, result: ActionResult) {
        if let Some(tx) = &self.completition_tx {
            let _ = tx.send(result);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ActionResult {
    Success,
    Failure,
}
