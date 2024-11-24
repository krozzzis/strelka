use core::document::{DocumentHandler, DocumentId};
use std::sync::Arc;

use crate::{Action, IntoAction, Receiver};

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
            content: Arc::new(self),
        }
    }
}
