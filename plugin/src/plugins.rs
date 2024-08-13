use std::sync::Arc;

use crate::{Plugin, PluginAction, PluginMessage};

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

    fn load(&mut self) -> Option<PluginAction> {
        println!("Example plugin loaded");
        None
    }

    fn unload(&mut self) -> Option<PluginAction> {
        println!("Example plugin unloaded");
        None
    }
}
