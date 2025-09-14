use std::path::PathBuf;
use std::sync::Arc;

use crate::BufferId;

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Close,
    ToggleMaximize,
    Collapse,
    DragStart,
    DragEnd,
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
