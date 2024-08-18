use std::path::PathBuf;

use crate::{document::DocumentStore, pane::PaneModel};

#[derive(Debug, Clone)]
pub struct State<'a, Content> {
    pub documents: &'a DocumentStore<Content>,
    pub panes: &'a PaneModel,
    pub working_directory: PathBuf,
}
