use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use strelka_api::message::WindowMessage;
use strelka_api::window::GuiService;

pub struct MessageBasedGuiService {
    command_sender: Sender<WindowMessage>,
}

impl MessageBasedGuiService {
    pub fn new(sender: Sender<WindowMessage>) -> Self {
        Self {
            command_sender: sender,
        }
    }
}

#[async_trait]
impl GuiService for MessageBasedGuiService {
    async fn send_window_message(&self, msg: WindowMessage) -> Result<(), String> {
        self.command_sender.try_send(msg).map_err(|e| e.to_string())
    }
}
