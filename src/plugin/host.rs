use std::{collections::HashMap, sync::Arc};

use crate::plugin::{self, Plugin};

#[derive(Default)]
pub struct PluginHost {
    pub plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin<S: Into<String>>(&mut self, name: S, mut plugin: Box<dyn Plugin>) {
        plugin.on_load();
        self.plugins.insert(name.into(), plugin);
    }

    pub fn send_action(&mut self, name: String, message: Arc<plugin::Action>) {
        if let Some(plugin) = self.plugins.get_mut(&name) {
            let _result = plugin.take_action(message);
        }
    }
}
