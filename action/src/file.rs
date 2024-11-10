use std::path::PathBuf;

use crate::{Action, IntoAction, Receiver};

#[derive(Debug, Clone)]
pub enum FileAction {
    PickFile,
    OpenFileCurrentTab(PathBuf),
    OpenFileForceCurrentTab(PathBuf),
    OpenFileNewTab(PathBuf),
}

impl IntoAction for FileAction {
    fn into_action(self) -> Action {
        Action {
            receiver: Receiver::File,
            content: Box::new(self),
            return_tx: None,
        }
    }
}
