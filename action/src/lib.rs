mod document;
mod file;
mod message;
mod pane;
mod theme;

use core::smol_str::SmolStr;
use std::any::Any;

pub use document::DocumentAction;
pub use file::FileAction;
pub use message::Message;
pub use pane::PaneAction;
pub use theme::ThemeAction;

use tokio::sync::oneshot;

#[derive(Debug)]
pub enum ActionResult {
    Value(Box<dyn Any + Send>),
    None,
    ReceiverNotFound,
    Failed,
}

pub trait IntoAction: Sized {
    fn into_action(self) -> Action;
    fn into_returnable_action(self) -> (Action, Option<oneshot::Receiver<ActionResult>>) {
        (Self::into_action(self), None)
    }
}

#[derive(Default, Debug, Clone)]
pub enum Receiver {
    #[default]
    Void,
    Document,
    File,
    Pane,
    Theme,
    Plugin(SmolStr),
}

#[derive(Debug)]
pub struct Action {
    pub receiver: Receiver,
    pub content: Box<dyn Any + Send>,
    pub return_tx: Option<oneshot::Sender<ActionResult>>,
}

impl IntoAction for Action {
    fn into_action(self) -> Action {
        self
    }
    fn into_returnable_action(self) -> (Action, Option<oneshot::Receiver<ActionResult>>) {
        (self, None)
    }
}
