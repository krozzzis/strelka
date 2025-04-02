mod theme_load;
pub use crate::theme_load::*;

use anyhow::{Result, anyhow};
use async_std::sync::RwLock;
use async_trait::async_trait;
use futures::future::BoxFuture;
use strelka_core::Message;
use std::collections::HashMap;
use std::sync::Arc;
use strelka_core::smol_str::SmolStr;

/// The context passed to commands when they are executed
#[derive(Debug, Clone)]
pub struct CommandArgs {
    // You can expand this with app state, editor state, etc.
    pub args: Vec<String>,
}

/// The trait that all commands must implement
#[async_trait]
pub trait Command: Send + Sync {
    /// Execute the command with the given context
    async fn execute(&self, args: CommandArgs) -> Result<Message>;
}

/// A registry for storing and retrieving commands
#[derive(Clone)]
pub struct CommandRegistry {
    commands: Arc<RwLock<HashMap<SmolStr, Arc<dyn Command>>>>,
}

impl CommandRegistry {
    /// Create a new empty command registry
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a command with the given name
    pub async fn register<C>(&self, name: impl Into<SmolStr>, command: C) -> Result<()>
    where
        C: Command + 'static,
    {
        let name = name.into();
        let mut commands = self.commands.write().await;

        if commands.contains_key(&name) {
            return Err(anyhow!("Command with name '{}' already exists", name));
        }

        commands.insert(name, Arc::new(command));
        Ok(())
    }

    /// Unregister a command with the given name
    pub async fn unregister(&self, name: &str) -> Result<()> {
        let mut commands = self.commands.write().await;

        if commands.remove(name).is_none() {
            return Err(anyhow!("Command with name '{}' does not exist", name));
        }

        Ok(())
    }

    /// Execute a command with the given name and context
    pub async fn execute(&self, name: &str, args: CommandArgs) -> Result<Message> {
        let commands = self.commands.read().await;

        let command = commands
            .get(name)
            .ok_or_else(|| anyhow!("Command with name '{}' does not exist", name))?;

        command.execute(args).await
    }

    /// Get a list of all registered command names
    pub async fn list_commands(&self) -> Vec<SmolStr> {
        let commands = self.commands.read().await;
        commands.iter().map(|(name, _cmd)| name.clone()).collect()
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
