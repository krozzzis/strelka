use core::action::Action;
use std::collections::HashMap;

use state::State;

use crate::{Plugin, PluginHandler, PluginInfo, PluginStatus};

pub type PluginId = String;

/// Plugin host
#[derive(Default)]
pub struct PluginHost {
    /// Registered plugins
    /// Stores plugin id and plugin handler,
    /// that contains plugin state and plugin status
    pub plugins: HashMap<PluginId, PluginHandler>,
}

#[allow(dead_code)]
impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
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
            state: plugin,
        };
        self.plugins.insert(id.clone(), handler);
    }

    pub fn process_action(&mut self, state: &State, action: Action) -> Action {
        let mut action = action;
        let ids: Vec<PluginId> = self.plugins.keys().cloned().collect();
        for id in ids {
            if let Some(handler) = self.plugins.get_mut(&id) {
                if handler.status == PluginStatus::Loaded {
                    action = handler.state.on_action(state, action);
                }
            }
        }
        action
    }
}
