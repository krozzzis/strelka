mod document;
mod file;
mod message;
mod pane;
mod theme;

pub use document::DocumentAction;
pub use file::FileAction;
pub use message::Message;
pub use pane::PaneAction;
pub use theme::ThemeAction;

use tokio::sync::broadcast;

pub trait IntoAction {
    fn into_action(self) -> Action;
}

#[derive(Debug, Clone)]
pub enum Action {
    File(FileAction),
    Pane(PaneAction),
    Document(DocumentAction),
    Theme(ThemeAction),
    Message(Message),
}

impl IntoAction for Action {
    fn into_action(self) -> Action {
        self
    }
}

#[derive(Debug)]
pub struct ActionWrapper {
    pub action: Action,
    pub completition_tx: Option<broadcast::Sender<ActionResult>>,
}

impl ActionWrapper {
    pub fn new(action: impl IntoAction) -> Self {
        Self {
            action: action.into_action(),
            completition_tx: None,
        }
    }

    pub fn notify(mut self, sender: broadcast::Sender<ActionResult>) -> Self {
        self.completition_tx = Some(sender);
        self
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn try_notify_complete(&self, result: ActionResult) {
        if let Some(tx) = &self.completition_tx {
            let _ = tx.send(result);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ActionResult {
    Success,
    Failure,
}
