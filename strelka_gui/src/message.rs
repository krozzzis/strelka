use crate::screen::{Screen, ScreenMessage};
use strelka_api::message::{CoreCommand, WindowMessage};

#[derive(Debug)]
pub enum Message {
    SetScreen(Box<Screen>),
    CoreCommand(CoreCommand),
    Screen(ScreenMessage),
    Window(WindowMessage),
    SetWindowId(iced::window::Id),
    None,
}
