use strelka_core::CoreCommand;

use crate::screen::{Screen, ScreenMessage};

#[derive(Debug)]
pub enum Message {
    SetScreen(Box<Screen>),
    CoreCommand(CoreCommand),
    Screen(ScreenMessage),
    Window(WindowMessage),
    SetWindowId(iced::window::Id),
    None,
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Close,
    ToggleMaximize,
    Collapse,
    DragStart,
    DragEnd,
}
