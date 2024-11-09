use action::{ActionWrapper, Message};
use std::{collections::HashMap, sync::Arc};

use log::warn;
use tokio::sync::{self, Mutex};

use crate::{MessageHandler, Plugin, PluginHandler, PluginInfo, PluginStatus};

pub type PluginId = String;

/// Plugin host
#[derive(Default)]
pub struct PluginHost {
    /// Registered plugins
    /// Stores plugin id and plugin handler,
    /// that contains plugin state and plugin status
    pub plugins: HashMap<PluginId, PluginHandler>,

    pub message_handlers: HashMap<PluginId, Arc<MessageHandler>>,

    pub brocker_tx: Option<sync::mpsc::Sender<ActionWrapper>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            message_handlers: HashMap::new(),
            brocker_tx: None,
        }
    }

    pub fn set_brocker(&mut self, brocker: sync::mpsc::Sender<ActionWrapper>) {
        self.brocker_tx = Some(brocker);
    }

    /// Returns list of registered plugin ids
    pub fn get_plugin_ids(&self) -> Vec<&String> {
        self.plugins.keys().collect()
    }

    /// Returns `Some(PluginStatus)` if plugin with given id is registered
    /// otherwise returns `None`
    pub fn get_plugin_status(&self, id: &str) -> Option<PluginStatus> {
        self.plugins.get(id).map(|handler| handler.status)
    }

    /// Returns `Some(PluginInfo)` if plugin with given id is registered
    /// otherwise returns `None`
    pub fn get_plugin_info(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id).map(|handler| &handler.info)
    }

    /// Load plugin with given id
    pub fn load_plugin(&mut self, id: &str) {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.status = PluginStatus::Loaded;
        }
    }

    /// Unload plugin with given id
    pub fn unload_plugin(&mut self, id: &str) {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.status = PluginStatus::Unloaded;
        }
    }

    /// Register plugin in host with given `id`, `PluginInfo` and State
    pub fn register_plugin(&mut self, info: PluginInfo, plugin: Box<dyn Plugin>) {
        let id = info.id.clone();
        let handler = PluginHandler {
            info,
            status: PluginStatus::Loaded,
            state: Arc::new(Mutex::new(plugin)),
        };
        self.plugins.insert(id.clone(), handler);
        self.init_message_handler(&id);
    }

    pub fn init_message_handler(&mut self, id: &PluginId) {
        if let Some(plugin_handler) = self.plugins.get(id) {
            let state = plugin_handler.state.blocking_lock();
            if let Some(message_handler) = state.create_message_handler() {
                self.message_handlers
                    .insert(id.clone(), Arc::new(message_handler));
            } else {
                self.message_handlers.remove(id);
            }
        }
    }

    pub async fn process_message(&self, message: Message) {
        let receiver = &message.destination;
        if let Some(handler) = &self.plugins.get(receiver) {
            if let PluginStatus::Loaded = handler.status {
                if let Some(message_handler) = self.message_handlers.get(receiver).cloned() {
                    let state = handler.state.clone();
                    let brocker = self.brocker_tx.clone();
                    tokio::spawn(async move { message_handler(state, message, brocker).await });
                } else {
                    warn!("No message handler found for processing message");
                }
            } else {
                warn!("Receiving plugin is not active");
            }
        } else {
            warn!("Receiving plugin does not registered")
        }
    }
}
