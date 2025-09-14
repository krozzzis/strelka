use std::pin::Pin;
use std::sync::Arc;

use dashmap::DashMap;

use strelka_api::core::CoreAPI;
use strelka_api::message::PluginMessage;

pub type ActionId = String;

pub type Handler = Arc<
    dyn Send
        + Sync
        + Fn(Arc<dyn CoreAPI>) -> Pin<Box<dyn Future<Output = PluginMessage> + Send + 'static>>,
>;

pub struct ActionRegistry {
    handlers: DashMap<ActionId, Handler>,
    core: Arc<dyn CoreAPI>,
}

impl ActionRegistry {
    pub fn new(core: Arc<dyn CoreAPI>) -> Self {
        Self {
            core,
            handlers: DashMap::new(),
        }
    }

    pub fn register<F, Fut>(&self, action_id: impl Into<ActionId>, handler: F)
    where
        F: Fn(Arc<dyn CoreAPI>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = PluginMessage> + Send + 'static,
    {
        self.handlers.insert(
            action_id.into(),
            Arc::new(move |core| {
                let fut = handler(core);
                Box::pin(fut) as _
            }),
        );
    }

    pub async fn execute(&self, action_id: impl Into<ActionId>) -> Option<PluginMessage> {
        let action_id = action_id.into();
        if let Some(h) = self.handlers.get(&action_id) {
            Some(h(self.core.clone()).await)
        } else {
            None
        }
    }
}
