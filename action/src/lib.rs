mod document;
mod file;
mod message;
mod pane;
mod theme;

use core::smol_str::SmolStr;
use std::{any::Any, sync::Arc};

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
    fn into_transport(self) -> ActionTransport {
        ActionTransport {
            action: self.into_action(),
            return_tx: None,
        }
    }
    fn into_transport_receive(self) -> (ActionTransport, oneshot::Receiver<ActionResult>) {
        let (tx, rx) = oneshot::channel();
        (
            ActionTransport {
                return_tx: Some(tx),
                ..self.into_transport()
            },
            rx,
        )
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

#[derive(Debug, Clone)]
pub struct Action {
    pub receiver: Receiver,
    pub content: Arc<dyn Any + Send + Sync>,
}

#[derive(Debug)]
pub struct ActionTransport {
    pub action: Action,
    pub return_tx: Option<oneshot::Sender<ActionResult>>,
}

impl IntoAction for Action {
    fn into_action(self) -> Action {
        self
    }

    fn into_transport(self) -> ActionTransport {
        ActionTransport {
            action: self,
            return_tx: None,
        }
    }
    fn into_transport_receive(self) -> (ActionTransport, oneshot::Receiver<ActionResult>) {
        let (tx, rx) = oneshot::channel();
        (
            ActionTransport {
                action: self,
                return_tx: Some(tx),
            },
            rx,
        )
    }
}
