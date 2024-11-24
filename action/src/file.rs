use std::{path::PathBuf, sync::Arc};

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
            content: Arc::new(self),
        }
    }
}
