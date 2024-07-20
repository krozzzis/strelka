mod host;
mod plugins;
mod plugins_list;

pub use host::*;
pub use plugins::*;
pub use plugins_list::*;

use std::sync::Arc;

#[derive(Debug, Default, Clone, Copy)]
pub enum PluginStatus {
    Loaded,
    #[default]
    Unloaded,
}

#[derive(Debug)]
pub struct Action {
    pub kind: String,
    pub payload: String,
}

impl Action {
    pub fn new(kind: String, payload: String) -> Self {
        Self { kind, payload }
    }
}

pub trait Plugin {
    fn take_action(&mut self, _action: Arc<Action>) -> Result<(), ()> {
        Ok(())
    }

    fn load(&mut self) {}
    fn unload(&mut self) {}
}

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

pub struct PluginHandler {
    pub info: PluginInfo,
    pub status: PluginStatus,
    pub state: Box<dyn Plugin>,
}
