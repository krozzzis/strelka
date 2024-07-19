use std::{collections::HashMap, sync::Arc};

use crate::plugin::{self, Plugin, PluginHandler, PluginInfo, PluginStatus};

pub type PluginId = String;

#[derive(Default)]
pub struct PluginHost {
    pub plugins: HashMap<PluginId, PluginHandler>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn get_plugin_ids(&self) -> Vec<String> {
        self.plugins
            .iter()
            .map(|(id, _handler)| id.clone())
            .collect()
    }

    /// Returns `Some(PluginStatus)` if plugin with given id is registered
    /// otherwise returns `None`
    pub fn get_plugin_status(&self, id: &str) -> Option<PluginStatus> {
        self.plugins.get(id).map(|handler| handler.status)
    }

    pub fn load_plugin(&mut self, id: &str) {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.state.load();
            plugin.status = PluginStatus::Loaded;
        }
    }

    pub fn unload_plugin(&mut self, id: &str) {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.state.unload();
            plugin.status = PluginStatus::Unloaded;
        }
    }

    /// Register plugin in host with given `id`, `PluginInfo` and State
    pub fn register_plugin<S: Into<String> + Clone>(
        &mut self,
        id: S,
        info: PluginInfo,
        mut plugin: Box<dyn Plugin>,
    ) {
        plugin.load();
        let handler = PluginHandler {
            info,
            status: PluginStatus::Loaded,
            state: plugin,
        };
        self.plugins.insert(id.into(), handler);
    }

    pub fn send_action(&mut self, name: String, action: Arc<plugin::Action>) {
        if let Some(plugin) = self.plugins.get_mut(&name) {
            let _result = plugin.state.take_action(action);
        }
    }
}
