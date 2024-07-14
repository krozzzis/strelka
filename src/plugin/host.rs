use std::{collections::HashMap, sync::Arc};

use crate::plugin::{self, Plugin};

#[derive(Default)]
pub struct PluginHost {
    pub plugins: HashMap<String, Box<dyn Plugin>>,
    pub plugins_loaded: HashMap<String, bool>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugins_loaded: HashMap::new(),
        }
    }

    pub fn get_plugin_names(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    pub fn is_plugin_loaded(&self, plugin: &str) -> Option<bool> {
        self.plugins_loaded.get(plugin).cloned()
    }

    pub fn load_plugin(&mut self, plugin: &str) {
        if let Some(plugin_load) = self.plugins_loaded.get_mut(plugin) {
            if let Some(plugin) = self.plugins.get_mut(plugin) {
                *plugin_load = true;
                plugin.on_load();
            }
        }
    }

    pub fn unload_plugin(&mut self, plugin: &str) {
        if let Some(plugin_load) = self.plugins_loaded.get_mut(plugin) {
            if let Some(plugin) = self.plugins.get_mut(plugin) {
                *plugin_load = false;
                plugin.on_unload();
            }
        }
    }

    pub fn register_plugin<S: Into<String> + Clone>(
        &mut self,
        name: S,
        mut plugin: Box<dyn Plugin>,
    ) {
        plugin.on_load();
        self.plugins.insert(name.clone().into(), plugin);
        self.plugins_loaded.insert(name.into(), true);
    }

    pub fn send_action(&mut self, name: String, message: Arc<plugin::Action>) {
        if let Some(plugin) = self.plugins.get_mut(&name) {
            let _result = plugin.take_action(message);
        }
    }
}
