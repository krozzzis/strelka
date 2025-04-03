use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use strelka_core::Theme as CoreTheme;
use strelka_core::command::{Command, CommandArgs};
use strelka_core::{Message, ThemeMessage};
use theming::Theme;
use theming::stylesheet::StyleSheet;

static DEFAULT_THEME_PATH: &str = "./themes/dark/theme.kdl";

#[derive(Default)]
pub struct ThemeLoadCommand {}

impl ThemeLoadCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for ThemeLoadCommand {
    async fn execute(&self, ctx: CommandArgs) -> Result<Message> {
        let path = PathBuf::from(
            ctx.args
                .first()
                .unwrap_or(&String::from(DEFAULT_THEME_PATH)),
        );
        println!("{path:?}");

        let sheet = StyleSheet::load(path).await.map_err(Error::msg)?;
        let theme = Theme::from_stylesheet(sheet);

        Ok(Message::Theme(ThemeMessage::SetTheme(CoreTheme {
            inner: Arc::new(theme),
        })))
    }
}
