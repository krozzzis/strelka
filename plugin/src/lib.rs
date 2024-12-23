mod host;
mod plugins;

use action::{Action, Message};
use std::{future::Future, pin::Pin, sync::Arc};

pub use host::*;
pub use plugins::*;
use tokio::sync::{mpsc, Mutex};

pub type MessageHandler = Box<
    dyn Fn(
            Arc<Mutex<Box<dyn Plugin>>>,
            Message,
            Option<mpsc::Sender<Action>>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

/// Plugin runtime status
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PluginStatus {
    Loaded,
    #[default]
    Unloaded,
}

/// Generic plugin trait
pub trait Plugin: Send + Sync {
    fn create_message_handler(&self) -> Option<MessageHandler> {
        None
    }

    fn on_action(&mut self, action: Action) -> Action {
        action
    }

    fn load(&mut self) {}

    fn unload(&mut self) {}
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
    pub state: Arc<Mutex<Box<dyn Plugin>>>,
}
