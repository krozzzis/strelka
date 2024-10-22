use std::{path::PathBuf, sync::Arc};

use tokio::sync::mpsc::Sender;

use crate::{
    document::{DocumentHandler, DocumentId},
    pane::{Pane, PaneId, VisiblePaneModel},
    ThemeID,
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
    Add(Pane, Option<Sender<PaneId>>),
    Replace(PaneId, Pane),
    GetOpen(Sender<Option<Pane>>),
    GetOpenId(Sender<Option<PaneId>>),
    GetModel(Sender<Option<VisiblePaneModel>>),
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
    Add(Arc<DocumentHandler<String>>, Option<Sender<DocumentId>>),
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
    SetTheme(ThemeID),
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

impl From<Message> for Action {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}
