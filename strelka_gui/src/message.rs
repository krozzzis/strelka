use strelka_core::CoreCommand;

use crate::screen::{Screen, ScreenMessage};

#[derive(Debug)]
pub enum Message {
    SetScreen(Box<Screen>),
    CoreCommand(CoreCommand),
    Screen(ScreenMessage),
    None,
}
