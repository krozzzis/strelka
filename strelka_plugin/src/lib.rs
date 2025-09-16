use std::pin::Pin;
use std::sync::Arc;

use dashmap::DashMap;

use strelka_api::Value;
use strelka_api::context::ActionContext;
use strelka_api::message::PluginMessage;

pub type ActionId = String;

pub type Handler = Arc<
    dyn Send
        + Sync
        + Fn(ActionContext, Value) -> Pin<Box<dyn Future<Output = PluginMessage> + Send + 'static>>,
>;

pub struct ActionRegistry {
    handlers: DashMap<ActionId, Handler>,
    context: ActionContext,
}

impl ActionRegistry {
    pub fn new(context: ActionContext) -> Self {
        Self {
            context,
            handlers: DashMap::new(),
        }
    }

    pub fn register<F, Fut>(&self, action_id: impl Into<ActionId>, handler: F)
    where
        F: Fn(ActionContext, Value) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = PluginMessage> + Send + 'static,
    {
        self.handlers.insert(
            action_id.into(),
            Arc::new(move |context, arg| {
                let fut = handler(context, arg);
                Box::pin(fut) as _
            }),
        );
    }

    pub async fn execute(
        &self,
        action_id: impl Into<ActionId>,
        arg: Value,
    ) -> Option<PluginMessage> {
        let action_id = action_id.into();
        if let Some(handler) = self.handlers.get(&action_id) {
            Some(handler(self.context.clone(), arg).await)
        } else {
            None
        }
    }
}
