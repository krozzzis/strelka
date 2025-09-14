use crate::message::{CoreAction, CoreMessage};
use async_trait::async_trait;

#[async_trait]
pub trait CoreAPI: Send + Sync {
    async fn handle_command(&self, cmd: CoreMessage) -> Option<CoreAction>;
}
