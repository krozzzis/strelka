use core::action::Action;

use tokio::sync::broadcast::Sender;

#[derive(Debug)]
pub struct ActionWrapper {
    pub action: Action,
    pub completition_tx: Option<Sender<ActionResult>>,
}

impl ActionWrapper {
    pub fn new(action: impl Into<Action>) -> Self {
        Self {
            action: action.into(),
            completition_tx: None,
        }
    }

    pub fn notify(mut self, sender: Sender<ActionResult>) -> Self {
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
