use crate::message::WindowMessage;

#[async_trait::async_trait]
pub trait GuiService: Send + Sync {
    async fn send_window_message(&self, msg: WindowMessage) -> Result<(), String>;
}
