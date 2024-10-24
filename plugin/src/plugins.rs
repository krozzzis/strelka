use crate::{MessageHandler, Plugin};

pub struct ExamplePlugin {}

unsafe impl Send for ExamplePlugin {}
unsafe impl Sync for ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn create_message_handler(&self) -> Option<MessageHandler> {
        Some(Box::new(|_plugin, message, _brocker| {
            Box::pin(async move {
                // Handle the message asynchronously
                println!("Received message: {:?}", message);
            })
        }))
    }

    fn load(&mut self) {
        println!("Example plugin loaded");
    }

    fn unload(&mut self) {
        println!("Example plugin unloaded");
    }
}
