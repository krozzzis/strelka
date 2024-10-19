use std::{path::PathBuf, sync::Arc};

use tokio::sync::mpsc::Sender;

use crate::{
    document::{DocumentHandler, DocumentId},
    pane::{Pane, PaneId, VisiblePaneModel},
    ThemeID,
};

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
pub enum GenericAction {
    File(FileAction),
    Pane(PaneAction),
    Document(DocumentAction),
    Theme(ThemeAction),
}

impl From<FileAction> for GenericAction {
    fn from(value: FileAction) -> Self {
        Self::File(value)
    }
}

impl From<PaneAction> for GenericAction {
    fn from(value: PaneAction) -> Self {
        Self::Pane(value)
    }
}

impl From<DocumentAction> for GenericAction {
    fn from(value: DocumentAction) -> Self {
        Self::Document(value)
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    pub actions: Vec<GenericAction>,
}

impl Action {
    pub fn new(action: impl Into<GenericAction>) -> Self {
        Self {
            actions: vec![action.into()],
        }
    }

    pub fn none() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(mut self, action: impl Into<GenericAction>) -> Self {
        self.actions.push(action.into());
        self
    }

    pub fn batch(actions: impl IntoIterator<Item = GenericAction>) -> Self {
        Self {
            actions: actions.into_iter().collect(),
        }
    }

    pub fn extend(mut self, actions: impl IntoIterator<Item = GenericAction>) -> Self {
        self.actions.extend(actions);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &GenericAction> {
        self.actions.iter()
    }
}

impl<'a> IntoIterator for &'a Action {
    type Item = &'a GenericAction;

    type IntoIter = std::slice::Iter<'a, GenericAction>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.iter()
    }
}
