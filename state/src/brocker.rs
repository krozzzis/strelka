use action::{ActionTransport, Receiver};

use log::info;
use tokio::sync::mpsc;

pub struct ActionBrocker {
    receiver: mpsc::Receiver<ActionTransport>,
    document_sender: Option<mpsc::Sender<ActionTransport>>,
    file_sender: Option<mpsc::Sender<ActionTransport>>,
    pane_sender: Option<mpsc::Sender<ActionTransport>>,
    theme_sender: Option<mpsc::Sender<ActionTransport>>,
    plugins_sender: Option<mpsc::Sender<ActionTransport>>,
}

impl ActionBrocker {
    pub fn new(rx: mpsc::Receiver<ActionTransport>) -> Self {
        Self {
            receiver: rx,
            document_sender: None,
            file_sender: None,
            pane_sender: None,
            theme_sender: None,
            plugins_sender: None,
        }
    }

    pub fn document_actor(mut self, document_tx: mpsc::Sender<ActionTransport>) -> Self {
        self.document_sender = Some(document_tx);
        self
    }

    pub fn file_actor(mut self, file_tx: mpsc::Sender<ActionTransport>) -> Self {
        self.file_sender = Some(file_tx);
        self
    }

    pub fn pane_actor(mut self, pane_tx: mpsc::Sender<ActionTransport>) -> Self {
        self.pane_sender = Some(pane_tx);
        self
    }

    pub fn theme_actor(mut self, theme_tx: mpsc::Sender<ActionTransport>) -> Self {
        self.theme_sender = Some(theme_tx);
        self
    }

    pub fn plugin_host_actor(mut self, plugins_tx: mpsc::Sender<ActionTransport>) -> Self {
        self.plugins_sender = Some(plugins_tx);
        self
    }

    pub async fn run(&mut self) {
        info!("Started Brocker's thread");
        while let Some(transport) = self.receiver.recv().await {
            info!("Brocker. Processing: {transport:?}");
            match transport.action.receiver {
                Receiver::Void => {}
                Receiver::Document => {
                    if let Some(tx) = &self.document_sender {
                        let _ = tx.send(transport).await;
                    }
                }
                Receiver::File => {
                    if let Some(tx) = &self.file_sender {
                        let _ = tx.send(transport).await;
                    }
                }
                Receiver::Pane => {
                    if let Some(tx) = &self.pane_sender {
                        let _ = tx.send(transport).await;
                    }
                }
                Receiver::Theme => {
                    if let Some(tx) = &self.theme_sender {
                        let _ = tx.send(transport).await;
                    }
                }
                Receiver::Plugin(_id) => todo!(),
            }
        }
    }
}
