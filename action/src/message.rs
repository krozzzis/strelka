use core::smol_str::SmolStr;

use crate::{Action, IntoAction, Receiver};

#[derive(Debug, Clone)]
pub struct Message {
    pub destination: String,
    pub kind: String,
    pub payload: Option<String>,
}

impl IntoAction for Message {
    fn into_action(self) -> Action {
        Action {
            receiver: Receiver::Plugin(SmolStr::new(self.destination.clone())),
            content: Box::new(self),
            return_tx: None,
        }
    }
}
