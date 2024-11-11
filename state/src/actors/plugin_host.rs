use action::Action;

use log::{info, warn};
use plugin::PluginHost;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PluginHostActor {
    plugin_host: PluginHost,
    receiver: Receiver<Action>,
    brocker_sender: Sender<Action>,
}

impl PluginHostActor {
    pub fn new(rx: Receiver<Action>, brocker_tx: Sender<Action>) -> Self {
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
        while let Some(generic_action) = self.receiver.recv().await {
            info!("PluginHostActor. Processing: {generic_action:?}");
            let action = if let Ok(x) = generic_action.content.downcast() {
                x
            } else {
                warn!("PluginHostActor. Dropping processing action because incorrect type");
                continue;
            };
            self.plugin_host.process_message(*action).await;
        }
    }
}
