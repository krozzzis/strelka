use std::{collections::HashMap, sync::Arc};

use crate::{Plugin, PluginAction, PluginHandler, PluginInfo, PluginMessage, PluginStatus};

pub type PluginId = String;

/// Plugin host
#[derive(Default)]
pub struct PluginHost<Message> {
    /// Registered plugins
    /// Stores plugin id and plugin handler,
    /// that contains plugin state and plugin status
    pub plugins: HashMap<PluginId, PluginHandler>,

    pub on_plugin_action: Option<Box<dyn Fn(PluginId, PluginAction) -> Message>>,
}

#[allow(dead_code)]
impl<Message> PluginHost<Message> {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            on_plugin_action: None,
        }
    }

    pub fn on_plugin_action<F>(mut self, func: F) -> Self
    where
        F: 'static + Fn(PluginId, PluginAction) -> Message,
    {
        self.on_plugin_action = Some(Box::new(func));
        self
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
    pub fn load_plugin(&mut self, id: &str) -> Option<Message> {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.status = PluginStatus::Loaded;
            if let Some(action) = plugin.state.load() {
                return self.map_action(id.to_string(), action);
            }
        }
        None
    }

    /// Unload plugin with given id
    pub fn unload_plugin(&mut self, id: &str) -> Option<Message> {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.status = PluginStatus::Unloaded;
            if let Some(action) = plugin.state.unload() {
                return self.map_action(id.to_string(), action);
            }
        }
        None
    }

    /// Convert plugin's `PluginAction` to application's `Message`
    fn map_action(&self, id: PluginId, action: PluginAction) -> Option<Message> {
        self.on_plugin_action
            .as_ref()
            .map(|mapper| mapper(id, action))
    }

    /// Register plugin in host with given `id`, `PluginInfo` and State
    pub fn register_plugin(&mut self, info: PluginInfo, plugin: Box<dyn Plugin>) {
        let id = info.id.clone();
        let handler = PluginHandler {
            info,
            status: PluginStatus::Loaded,
            state: plugin,
        };
        self.plugins.insert(id.clone(), handler);
    }

    /// Send a message to plugin with given `PluginId`.
    ///
    /// Optionally returns an Application's message, if plugin return an action.
    pub fn send_message(&mut self, id: PluginId, message: Arc<PluginMessage>) -> Option<Message> {
        if let Some(plugin) = self.plugins.get_mut(&id) {
            if let PluginStatus::Loaded = plugin.status {
                let result = plugin.state.update(message);
                if let Some(action) = result {
                    return self.map_action(id, action);
                }
            }
        }
        None
    }
}
