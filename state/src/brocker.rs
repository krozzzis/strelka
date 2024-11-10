use action::{Action, FileAction};

use log::info;
use tokio::sync::{
    broadcast,
    mpsc::{Receiver, Sender},
};

pub struct ActionBrocker {
    receiver: Receiver<Action>,
    document_sender: Option<Sender<Action>>,
    file_sender: Option<Sender<Action>>,
    pane_sender: Option<Sender<Action>>,
    theme_sender: Option<Sender<Action>>,
    plugins_sender: Option<Sender<Action>>,
}

impl ActionBrocker {
    pub fn new(rx: Receiver<Action>) -> Self {
        Self {
            receiver: rx,
            document_sender: None,
            file_sender: None,
            pane_sender: None,
            theme_sender: None,
            plugins_sender: None,
        }
    }

    pub fn document_actor(mut self, document_tx: Sender<Action>) -> Self {
        self.document_sender = Some(document_tx);
        self
    }

    pub fn file_actor(mut self, file_tx: Sender<Action>) -> Self {
        self.file_sender = Some(file_tx);
        self
    }

    pub fn pane_actor(mut self, pane_tx: Sender<Action>) -> Self {
        self.pane_sender = Some(pane_tx);
        self
    }

    pub fn theme_actor(mut self, theme_tx: Sender<Action>) -> Self {
        self.theme_sender = Some(theme_tx);
        self
    }

    pub fn plugin_host_actor(mut self, plugins_tx: Sender<Action>) -> Self {
        self.plugins_sender = Some(plugins_tx);
        self
    }

    pub async fn run(&mut self) {
        info!("Started Brocker's thread");
        while let Some(action) = self.receiver.recv().await {
            info!("Brocker. Processing: {action:?}");
            match action.receiver {
                action::Receiver::Void => {},
                action::Receiver::Document => {
                    if let Some(tx) = self.document_sender {
                        tx.send(action.content).await
                    }
                }
                action::Receiver::File => todo!(),
                action::Receiver::Pane => todo!(),
                action::Receiver::Theme => todo!(),
                action::Receiver::Plugin(id) => todo!(),
            }
        }
    }
}
