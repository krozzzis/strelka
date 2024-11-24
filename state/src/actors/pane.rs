use action::{ActionResult, ActionTransport, DocumentAction, IntoAction, PaneAction};
use core::pane::{Pane, PaneModel, VisiblePaneModel};
use std::sync::Arc;

use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PaneActor {
    panes: PaneModel,
    receiver: Receiver<ActionTransport>,
    brocker_sender: Sender<ActionTransport>,
}

impl PaneActor {
    pub fn new(rx: Receiver<ActionTransport>, brocker_tx: Sender<ActionTransport>) -> Self {
        Self {
            panes: PaneModel::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started PaneActor");
        while let Some(transport) = self.receiver.recv().await {
            info!("PaneActor. Processing: {transport:?}");
            let action: Arc<PaneAction> = if let Ok(x) = transport.action.content.downcast() {
                x
            } else {
                warn!("PaneActor. Dropping processing action because incorrect type");
                continue;
            };
            match action.as_ref() {
                PaneAction::Close(id) => {
                    let pane = self.panes.remove(id);

                    // Close document if Editor pane was closed
                    if let Some(Pane::Editor(doc_id)) = pane {
                        let message = DocumentAction::Remove(doc_id).into_transport();
                        let _ = self.brocker_sender.send(message).await;
                    }

                    // If there no panes left, create a NewDocument one
                    if self.panes.count() == 0 {
                        let id = self.panes.add(Pane::NewDocument);
                        self.panes.open(&id);
                    }
                }
                PaneAction::Open(id) => {
                    self.panes.open(id);
                }
                PaneAction::Add(pane) => {
                    let id = self.panes.add(*pane);
                    self.panes.open(&id);
                    if let Some(tx) = transport.return_tx {
                        let _ = tx.send(ActionResult::Value(Box::new(id)));
                    }
                }
                PaneAction::Replace(id, pane) => {
                    self.panes.replace(id, *pane);
                }
                PaneAction::GetOpen() => {
                    if let Some(tx) = transport.return_tx {
                        let opened = self.panes.get_open().cloned();
                        let _ = tx.send(ActionResult::Value(Box::new(opened)));
                    }
                }
                PaneAction::GetOpenId() => {
                    if let Some(tx) = transport.return_tx {
                        let opened_id = self.panes.get_open_id().cloned();
                        let _ = tx.send(ActionResult::Value(Box::new(opened_id)));
                    }
                }
                PaneAction::GetModel() => {
                    if let Some(tx) = transport.return_tx {
                        let opened_id = self.panes.get_open_id().cloned();
                        let panes = self.panes.get_visible_panes();
                        let model = VisiblePaneModel {
                            panes,
                            opened: opened_id,
                        };
                        let _ = tx.send(ActionResult::Value(Box::new(Some(model))));
                    }
                }
            }
        }
    }
}
