use tokio::sync::mpsc::Sender;

use strelka_api::Value;
use strelka_api::message::{CoreMessage, WindowMessage};
use strelka_plugin::ActionId;

use crate::widget::file_manager;

#[derive(Debug, Clone)]
pub enum Message {
    CoreMessage(CoreMessage),
    Window(WindowMessage),
    Action(ActionId, Value),
    FileManager(file_manager::Message),
    GUIChannelEstablised(Sender<WindowMessage>),
    None,
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        Message::Action(String::from(s), Value::None)
    }
}
