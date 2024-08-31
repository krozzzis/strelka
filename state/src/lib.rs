use std::path::PathBuf;

use core::{document::DocumentStore, pane::PaneModel};
use theming::catalog::{Catalog, ThemeID};

#[derive(Clone)]
pub struct State<'a, Content> {
    pub documents: &'a DocumentStore<Content>,
    pub panes: &'a PaneModel,
    pub theme: &'a ThemeID,
    pub themes: &'a Catalog,
    pub working_directory: PathBuf,
}
