use core::pane::{Pane, PaneId};

use tokio::sync::oneshot;

use crate::{Action, ActionResult, IntoAction, Receiver};

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
            content: Box::new(self),
            return_tx: None,
        }
    }

    fn into_returnable_action(self) -> (Action, Option<oneshot::Receiver<ActionResult>>) {
        match &self {
            PaneAction::Add(_) => {
                let (tx, rx) = oneshot::channel();
                let action = Action {
                    return_tx: Some(tx),
                    ..Self::into_action(self)
                };
                (action, Some(rx))
            }
            PaneAction::GetOpen() => {
                let (tx, rx) = oneshot::channel();
                let action = Action {
                    return_tx: Some(tx),
                    ..Self::into_action(self)
                };
                (action, Some(rx))
            }
            PaneAction::GetOpenId() => {
                let (tx, rx) = oneshot::channel();
                let action = Action {
                    return_tx: Some(tx),
                    ..Self::into_action(self)
                };
                (action, Some(rx))
            }
            PaneAction::GetModel() => {
                let (tx, rx) = oneshot::channel();
                let action = Action {
                    return_tx: Some(tx),
                    ..Self::into_action(self)
                };
                (action, Some(rx))
            }
            _ => (self.into_action(), None),
        }
    }
}
