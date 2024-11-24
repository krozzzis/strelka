use std::sync::Arc;

use action::{ActionTransport, Message};

use log::{info, warn};
use plugin::PluginHost;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PluginHostActor {
    plugin_host: PluginHost,
    receiver: Receiver<ActionTransport>,
    brocker_sender: Sender<ActionTransport>,
}

impl PluginHostActor {
    pub fn new(rx: Receiver<ActionTransport>, brocker_tx: Sender<ActionTransport>) -> Self {
        Self {
            plugin_host: PluginHost::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub fn set_host(mut self, host: PluginHost) -> Self {
        self.plugin_host = host;
        self
    }

    pub async fn run(&mut self) {
        info!("Started PluginHostActor");
        while let Some(transport) = self.receiver.recv().await {
            info!("PluginHostActor. Processing: {transport:?}");
            let action: Arc<Message> = if let Ok(x) = transport.action.content.downcast() {
                x
            } else {
                warn!("PluginHostActor. Dropping processing action because incorrect type");
                continue;
            };
            self.plugin_host.process_message(action).await;
        }
    }
}
