use core::document::{DocumentHandler, DocumentId};
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{Action, IntoAction};

#[derive(Debug, Clone)]
pub enum DocumentAction {
    Add(
        Arc<DocumentHandler<String>>,
        Option<mpsc::Sender<DocumentId>>,
    ),
    Open(DocumentId),
    Save(DocumentId),
    Remove(DocumentId),
}

impl IntoAction for DocumentAction {
    fn into_action(self) -> Action {
        Action::Document(self)
    }
}
