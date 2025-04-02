use crate::{smol_str::SmolStr, theme::Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Command(CommandMessage),
    Theme(ThemeMessage),
    None,
}

#[derive(Debug, Clone)]
pub enum CommandMessage {
    CallCommand(SmolStr, Vec<String>),
}

#[derive(Debug, Clone)]
pub enum ThemeMessage {
    SetTheme(Theme),
}
