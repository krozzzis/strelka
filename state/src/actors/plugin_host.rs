use core::action::{Action, ActionWrapper};

use log::{info, warn};
use plugin::PluginHost;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PluginHostActor {
    plugin_host: PluginHost,
    receiver: Receiver<ActionWrapper>,
    brocker_sender: Sender<ActionWrapper>,
}

impl PluginHostActor {
    pub fn new(rx: Receiver<ActionWrapper>, brocker_tx: Sender<ActionWrapper>) -> Self {
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
        info!("PluginHostActor. Started thread");
        while let Some(wrapper) = self.receiver.recv().await {
            info!("PluginHostActor. Processing: {wrapper:?}");
            let message = if let Action::Message(message) = wrapper.action {
                message
            } else {
                warn!("PluginHostActor. Dropping processing action because incorrect type");
                continue;
            };
            self.plugin_host.process_message(message).await;
        }
    }
}
