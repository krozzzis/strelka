mod host;
mod plugins;

pub use host::*;
pub use plugins::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct Action {
    pub kind: String,
    pub payload: String,
}

impl Action {
    pub fn new(kind: String, payload: String) -> Self {
        Self { kind, payload }
    }
}

pub trait Plugin {
    fn take_action(&mut self, _action: Arc<Action>) -> Result<(), ()> {
        Ok(())
    }

    fn on_load(&mut self) {}
}
