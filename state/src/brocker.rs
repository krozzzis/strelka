use core::action::{Action, ActionWrapper, FileAction};

use log::info;
use tokio::sync::{
    broadcast,
    mpsc::{Receiver, Sender},
};

pub struct ActionBrocker {
    receiver: Receiver<ActionWrapper>,
    document_sender: Option<Sender<ActionWrapper>>,
    file_sender: Option<Sender<FileAction>>,
    pane_sender: Option<Sender<ActionWrapper>>,
    plugins_sender: Option<Sender<ActionWrapper>>,
}

impl ActionBrocker {
    pub fn new(rx: Receiver<ActionWrapper>) -> Self {
        Self {
            receiver: rx,
            document_sender: None,
            file_sender: None,
            pane_sender: None,
            plugins_sender: None,
        }
    }

    pub fn document_actor(mut self, document_tx: Sender<ActionWrapper>) -> Self {
        self.document_sender = Some(document_tx);
        self
    }

    pub fn file_actor(mut self, file_tx: Sender<FileAction>) -> Self {
        self.file_sender = Some(file_tx);
        self
    }

    pub fn pane_actor(mut self, pane_tx: Sender<ActionWrapper>) -> Self {
        self.pane_sender = Some(pane_tx);
        self
    }

    pub fn plugin_host_actor(mut self, plugins_tx: Sender<ActionWrapper>) -> Self {
        self.plugins_sender = Some(plugins_tx);
        self
    }

    pub async fn run(&mut self) {
        info!("Started Brocker's thread");
        while let Some(wrapper) = self.receiver.recv().await {
            info!("Brocker. Processing: {wrapper:?}");
            match wrapper.action() {
                Action::File(action) => {
                    if let Some(tx) = &self.file_sender {
                        let _ = tx.send(action.clone()).await;
                    }
                }
                Action::Pane(action) => {
                    if let Some(tx) = &self.pane_sender {
                        let (complete_tx, mut complete_rx) = broadcast::channel(1);
                        let message =
                            ActionWrapper::new(Action::Pane(action.clone())).notify(complete_tx);
                        let _ = tx.send(message).await;

                        // ActionResult throwing
                        while let Ok(result) = complete_rx.recv().await {
                            info!("Brocker. Received {result:?} from PaneActor");
                            wrapper.try_notify_complete(result);
                        }
                    }
                }
                Action::Document(action) => {
                    if let Some(tx) = &self.document_sender {
                        let (complete_tx, mut complete_rx) = broadcast::channel(1);
                        let message = ActionWrapper::new(Action::Document(action.clone()))
                            .notify(complete_tx);
                        let _ = tx.send(message).await;

                        // ActionResult throwing
                        while let Ok(result) = complete_rx.recv().await {
                            info!("Brocker. Received {result:?} from DocumentActor");
                            wrapper.try_notify_complete(result);
                        }
                    }
                }
                Action::Message(action) => {
                    if let Some(tx) = &self.plugins_sender {
                        let (complete_tx, mut complete_rx) = broadcast::channel(1);
                        let message =
                            ActionWrapper::new(Action::Message(action.clone())).notify(complete_tx);
                        let _ = tx.send(message).await;

                        // ActionResult throwing
                        while let Ok(result) = complete_rx.recv().await {
                            info!("Brocker. Received {result:?} from PluginHostActor");
                            wrapper.try_notify_complete(result);
                        }
                    }
                }
                Action::Theme(_) => todo!(),
            }
        }
    }
}
