use std::{collections::HashMap, sync::Arc};

use crate::plugin::{
    self, Plugin, PluginAction, PluginEntry, PluginHandler, PluginInfo, PluginStatus,
};

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
        self.plugins.iter().map(|(id, _handler)| id).collect()
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

    fn map_action(&self, id: PluginId, action: PluginAction) -> Option<Message> {
        if let Some(mapper) = &self.on_plugin_action {
            Some(mapper(id, action))
        } else {
            None
        }
    }

    /// Register plugin in host with given `id`, `PluginInfo` and State
    pub fn register_plugin(&mut self, info: PluginInfo, mut plugin: Box<dyn Plugin>) {
        let id = info.id.clone();
        let handler = PluginHandler {
            info,
            status: PluginStatus::Loaded,
            state: plugin,
        };
        self.plugins.insert(id.clone(), handler);
    }

    pub fn send_message(
        &mut self,
        id: String,
        message: Arc<plugin::PluginMessage>,
    ) -> Option<Message> {
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

    /// Returns list of registered plugins
    pub fn get_plugin_entries(&self) -> Vec<PluginEntry> {
        let mut result = Vec::new();
        for (_id, handler) in self.plugins.iter() {
            let entry = PluginEntry {
                info: &handler.info,
                status: handler.status,
            };
            result.push(entry);
        }
        result.sort_unstable_by(|a, b| a.info.name.cmp(&b.info.name));

        result
    }
}
