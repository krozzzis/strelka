use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use strelka_core::Message;

use crate::{Command, CommandArgs};

static DEFAULT_THEME_PATH: &str = "./themes/dark/theme.kdl";

// Example async command implementation
pub struct ThemeLoadCommand {}

impl ThemeLoadCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for ThemeLoadCommand {
    async fn execute(&self, ctx: CommandArgs) -> Result<Message> {
        let path = PathBuf::from(ctx.args.get(0).unwrap_or(&String::from(DEFAULT_THEME_PATH)));
        println!("{path:?}");

        Ok(Message::None)
    }
}
