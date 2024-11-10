use core::document::{DocumentHandler, DocumentId};
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot};

use crate::{Action, ActionResult, IntoAction, Receiver};

#[derive(Debug, Clone)]
pub enum DocumentAction {
    Add(Arc<DocumentHandler<String>>),
    Open(DocumentId),
    Save(DocumentId),
    Remove(DocumentId),
}

impl IntoAction for DocumentAction {
    fn into_action(self) -> Action {
        Action {
            receiver: Receiver::Document,
            content: Box::new(self),
            return_tx: None,
        }
    }

    fn into_returnable_action(
        self,
    ) -> (
        Action,
        Option<tokio::sync::oneshot::Receiver<ActionResult>>,
    ) {
        match &self {
            DocumentAction::Add(_) => {
                let (tx, rx) = oneshot::channel();
                let action = Action {
                    return_tx: Some(tx),
                    ..Self::into_action(self)
                };
                (action, Some(rx))
            }
            DocumentAction::Open(_) => (Self::into_action(self), None),
            DocumentAction::Save(_) => (Self::into_action(self), None),
            DocumentAction::Remove(_) => (Self::into_action(self), None),
        }
    }
}
