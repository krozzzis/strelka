use core::pane::{Pane, PaneId};
use std::sync::Arc;

use crate::{Action, IntoAction, Receiver};

#[derive(Debug, Clone)]
pub enum PaneAction {
    Close(PaneId),
    Open(PaneId),
    Add(Pane),
    Replace(PaneId, Pane),
    GetOpen(),
    GetOpenId(),
    GetModel(),
}

impl IntoAction for PaneAction {
    fn into_action(self) -> Action {
        Action {
            receiver: Receiver::Pane,
            content: Arc::new(self),
        }
    }
}
