use std::sync::Arc;

use crate::plugin::{self, Plugin};

pub struct ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn take_action(&mut self, message: Arc<plugin::Action>) -> Result<(), ()> {
        if message.kind == "say" {
            println!("{}", message.payload);
            Ok(())
        } else {
            Err(())
        }
    }

    fn on_load(&mut self) {
        println!("Example plugin loaded");
    }
}
