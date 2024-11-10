use std::path::PathBuf;

use crate::{Action, IntoAction};

#[derive(Debug, Clone)]
pub enum FileAction {
    PickFile,
    OpenFileCurrentTab(PathBuf),
    OpenFileForceCurrentTab(PathBuf),
    OpenFileNewTab(PathBuf),
}

impl IntoAction for FileAction {
    fn into_action(self) -> Action {
        Action::File(self)
    }
}
