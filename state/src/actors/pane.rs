use core::{
    action::{Action, ActionResult, ActionWrapper, DocumentAction, PaneAction},
    pane::{Pane, PaneModel, VisiblePaneModel},
};

use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PaneActor {
    panes: PaneModel,
    receiver: Receiver<ActionWrapper>,
    brocker_sender: Sender<ActionWrapper>,
}

impl PaneActor {
    pub fn new(rx: Receiver<ActionWrapper>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            panes: PaneModel::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("PaneActors. Started thread");
        while let Some(wrapper) = self.receiver.recv().await {
            info!("PaneActor. Processing: {wrapper:?}");
            let action = if let Action::Pane(action) = wrapper.action() {
                action
            } else {
                warn!("PaneActor. Dropping processing action because incorrect type");
                continue;
            };
            match action {
                PaneAction::Close(id) => {
                    let pane = self.panes.remove(id);

                    // Close document if Editor pane was closed
                    if let Some(Pane::Editor(doc_id)) = pane {
                        let message = ActionWrapper::new(DocumentAction::Remove(doc_id));
                        let _ = self.brocker_sender.send(message).await;
                    }

                    // If there no panes left, create a NewDocument one
                    if self.panes.count() == 0 {
                        let id = self.panes.add(Pane::NewDocument);
                        self.panes.open(&id);
                    }

                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::Open(id) => {
                    self.panes.open(id);
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::Add(pane, tx) => {
                    let id = self.panes.add(*pane);
                    self.panes.open(&id);
                    if let Some(tx) = tx {
                        let _ = tx.send(id).await;
                    }
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::Replace(id, pane) => {
                    self.panes.replace(id, *pane);
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::GetOpen(tx) => {
                    let opened = self.panes.get_open().cloned();
                    let _ = tx.send(opened).await;
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::GetOpenId(tx) => {
                    let opened_id = self.panes.get_open_id().cloned();
                    let _ = tx.send(opened_id).await;
                    wrapper.try_notify_complete(ActionResult::Success);
                }
                PaneAction::GetModel(tx) => {
                    let opened_id = self.panes.get_open_id().cloned();
                    let panes = self.panes.get_visible_panes();
                    let model = VisiblePaneModel {
                        panes,
                        opened: opened_id,
                    };
                    let _ = tx.send(Some(model)).await;
                    wrapper.try_notify_complete(ActionResult::Success);
                }
            }
        }
    }
}
