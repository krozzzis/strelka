use strelka_api::Value;
use strelka_api::message::{CoreMessage, WindowMessage};
use strelka_plugin::ActionId;

use crate::screen::{Screen, ScreenMessage};

#[derive(Debug, Clone)]
pub enum Message {
    SetScreen(Box<Screen>),
    CoreMessage(CoreMessage),
    Screen(ScreenMessage),
    Window(WindowMessage),
    SetWindowId(iced::window::Id),
    Action(ActionId, Value),
    None,
}
