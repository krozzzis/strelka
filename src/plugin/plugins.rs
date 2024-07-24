use std::sync::Arc;

use crate::plugin::{self, Plugin, PluginAction, PluginMessage};

pub struct ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn update(&mut self, message: Arc<PluginMessage>) -> Option<PluginAction> {
        if message.kind == "say" {
            println!("{}", message.payload);
            Some(PluginAction::SendNotification(Arc::new(format!(
                "Message from plugin: {}",
                message.payload
            ))))
        } else {
            None
        }
    }

    fn load(&mut self) {
        println!("Example plugin loaded");
    }

    fn unload(&mut self) {
        println!("Example plugin unloaded");
    }
}
