mod host;
mod plugins;

pub use host::*;
pub use plugins::*;

use std::sync::Arc;

/// Plugin runtime status
#[derive(Debug, Default, Clone, Copy)]
pub enum PluginStatus {
    Loaded,
    #[default]
    Unloaded,
}

/// Plugin action
///
/// Used for sending messages from application to plugin
#[derive(Debug, Clone)]
pub struct PluginMessage {
    pub kind: String,
    pub payload: String,
}

impl PluginMessage {
    pub fn new(kind: String, payload: String) -> Self {
        Self { kind, payload }
    }
}

/// Generic plugin trait
pub trait Plugin {
    fn update(&mut self, _message: Arc<PluginMessage>) -> Option<PluginAction> {
        None
    }

    fn load(&mut self) -> Option<PluginAction> {
        None
    }
    fn unload(&mut self) -> Option<PluginAction> {
        None
    }
}

/// Plugin information
#[derive(Debug, Default, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub id: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub version: Option<String>,
}

impl PluginInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }

    pub fn author<S: Into<String>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn license<S: Into<String>>(mut self, license: S) -> Self {
        self.license = Some(license.into());
        self
    }

    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = Some(version.into());
        self
    }
}

/// Plugin handler
///
/// Stores info, plugin state and plugin status
pub struct PluginHandler {
    pub info: PluginInfo,
    pub status: PluginStatus,
    pub state: Box<dyn Plugin>,
}

#[derive(Debug, Clone)]
pub enum PluginAction {
    SendNotification(Arc<String>),
}