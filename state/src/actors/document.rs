use action::{Action, ActionResult, DocumentAction, IntoAction, PaneAction};
use core::{
    document::{DocumentHandler, DocumentStore},
    pane::Pane,
};

use iced::widget::text_editor::Content;
use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct DocumentActor {
    documents: DocumentStore<Content>,
    receiver: Receiver<Action>,
    brocker_sender: Sender<Action>,
}

impl DocumentActor {
    pub fn new(rx: Receiver<Action>, brocker_tx: Sender<Action>) -> Self {
        Self {
            documents: DocumentStore::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started DocumentActor");
        while let Some(generic_action) = self.receiver.recv().await {
            info!("DocumentActor. Processing: {generic_action:?}");
            let action = if let Ok(x) = generic_action.content.downcast() {
                x
            } else {
                warn!("DocumentActor. Dropping processing action because incorrect type");
                continue;
            };
            match *action {
                DocumentAction::Add(handler) => {
                    let content = Content::with_text(&handler.text_content);
                    let handler = DocumentHandler {
                        text_content: content,
                        path: handler.path.clone(),
                        filename: handler.filename.clone(),
                        changed: handler.changed,
                    };
                    let doc_id = self.documents.add(handler);
                    if let Some(tx) = generic_action.return_tx {
                        let result = ActionResult::Value(Box::new(doc_id));
                        info!("Return {result:?}");
                        let _ = tx.send(result);
                    }
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(id);
                    let message = PaneAction::Add(pane).into_action();
                    let _ = self.brocker_sender.send(message).await;
                }
                DocumentAction::Save(_id) => {
                    todo!()
                }
                DocumentAction::Remove(id) => {
                    self.documents.remove(&id);
                }
            }
        }
    }
}
