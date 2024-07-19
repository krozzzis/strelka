use crate::plugin::PluginInfo;

pub struct PluginsList<'a> {
    pub plugins: &'a Vec<PluginInfo>,
}

pub fn plugin_list(plugins: &Vec<PluginInfo>) -> PluginsList {
    PluginsList { plugins }
}
