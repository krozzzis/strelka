use tokio::sync::mpsc::Sender;

use strelka_api::Value;
use strelka_api::message::{CoreMessage, WindowMessage};
use strelka_plugin::ActionId;

use crate::screen::{Screen, ScreenMessage};

#[derive(Debug, Clone)]
pub enum Message {
    CoreMessage(CoreMessage),
    Screen(ScreenMessage),
    Window(WindowMessage),
    Action(ActionId, Value),
    GUIChannelEstablised(Sender<WindowMessage>),
    None,
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        Message::Action(String::from(s), Value::None)
    }
}
