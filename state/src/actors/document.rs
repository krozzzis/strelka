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
    receiver: Receiver<ActionWrapper>,
    brocker_sender: Sender<ActionWrapper>,
}

impl DocumentActor {
    pub fn new(rx: Receiver<ActionWrapper>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            documents: DocumentStore::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started DocumentActor's thread");
        while let Some(wrapper) = self.receiver.recv().await {
            info!("DocumentActor. Processing: {wrapper:?}");
            let action = if let Action::Document(action) = wrapper.action() {
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
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(*id);
                    let message = ActionWrapper::new(PaneAction::Add(pane, None));
                    let _ = self.brocker_sender.send(message).await;
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                DocumentAction::Save(_id) => {
                    todo!()
                }
                DocumentAction::Remove(id) => {
                    self.documents.remove(id);
                    wrapper.try_notify_complete(ActionResult::Success);
                }
            }
        }
    }
}
