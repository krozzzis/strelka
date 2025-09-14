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
pub enum CoreAction {
    DocumentOpened(BufferId),
    None,
}

#[derive(Debug, Clone)]
pub enum CoreMessage {
    InsertText(BufferId, Arc<String>),
    OpenFile(PathBuf),
}

#[derive(Debug, Clone)]
pub enum PluginMessage {
    Core(CoreMessage),
    Window(WindowMessage),
    None,
}
