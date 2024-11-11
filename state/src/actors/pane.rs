use action::{Action, DocumentAction, IntoAction, PaneAction};
use core::pane::{Pane, PaneModel, VisiblePaneModel};

use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PaneActor {
    panes: PaneModel,
    receiver: Receiver<Action>,
    brocker_sender: Sender<Action>,
}

impl PaneActor {
    pub fn new(rx: Receiver<Action>, brocker_tx: Sender<Action>) -> Self {
        Self {
            panes: PaneModel::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started PaneActor");
        while let Some(generic_action) = self.receiver.recv().await {
            info!("PaneActor. Processing: {generic_action:?}");
            let action = if let Ok(x) = generic_action.content.downcast() {
                x
            } else {
                warn!("PaneActor. Dropping processing action because incorrect type");
                continue;
            };
            match *action {
                PaneAction::Close(id) => {
                    let pane = self.panes.remove(&id);

                    // Close document if Editor pane was closed
                    if let Some(Pane::Editor(doc_id)) = pane {
                        let message: Action = DocumentAction::Remove(doc_id).into_action();
                        let _ = self.brocker_sender.send(message).await;
                    }

                    // If there no panes left, create a NewDocument one
                    if self.panes.count() == 0 {
                        let id = self.panes.add(Pane::NewDocument);
                        self.panes.open(&id);
                    }
                }
                PaneAction::Open(id) => {
                    self.panes.open(&id);
                }
                PaneAction::Add(pane, tx) => {
                    let id = self.panes.add(pane);
                    self.panes.open(&id);
                    if let Some(tx) = tx {
                        let _ = tx.send(id).await;
                    }
                }
                PaneAction::Replace(id, pane) => {
                    self.panes.replace(&id, pane);
                }
                PaneAction::GetOpen(tx) => {
                    let opened = self.panes.get_open().cloned();
                    let _ = tx.send(opened).await;
                }
                PaneAction::GetOpenId(tx) => {
                    let opened_id = self.panes.get_open_id().cloned();
                    let _ = tx.send(opened_id).await;
                }
                PaneAction::GetModel(tx) => {
                    let opened_id = self.panes.get_open_id().cloned();
                    let panes = self.panes.get_visible_panes();
                    let model = VisiblePaneModel {
                        panes,
                        opened: opened_id,
                    };
                    let _ = tx.send(Some(model)).await;
                }
            }
        }
    }
}
