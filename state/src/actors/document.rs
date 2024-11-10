use action::{Action, ActionResult, ActionWrapper, DocumentAction, PaneAction};
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
        while let Some(action) = self.receiver.recv().await {
            info!("DocumentActor. Processing: {action:?}");
            let action = if let Action::Document(action) = action.action() {
                action
            } else {
                warn!("DocumentActor. Dropping processing action because incorrect type");
                continue;
            };
            match action {
                DocumentAction::Add(handler, tx) => {
                    let content = Content::with_text(&handler.text_content);
                    let handler = DocumentHandler {
                        text_content: content,
                        path: handler.path.clone(),
                        filename: handler.filename.clone(),
                        changed: handler.changed,
                    };
                    let doc_id = self.documents.add(handler);
                    if let Some(tx) = tx {
                        let _ = tx.send(doc_id).await;
                    }

                    info!("DocumentActor. Sending sucess");
                    action.try_notify_complete(ActionResult::Success);
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(*id);
                    let message = ActionWrapper::new(PaneAction::Add(pane, None));
                    let _ = self.brocker_sender.send(message).await;
                    action.try_notify_complete(ActionResult::Success);
                }
                DocumentAction::Save(_id) => {
                    todo!()
                }
                DocumentAction::Remove(id) => {
                    self.documents.remove(id);
                    action.try_notify_complete(ActionResult::Success);
                }
            }
        }
    }
}
