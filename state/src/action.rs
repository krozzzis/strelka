use core::action::GenericAction;

use tokio::sync::broadcast::Sender;

#[derive(Debug)]
pub struct ActionWrapper {
    pub action: GenericAction,
    pub completition_tx: Option<Sender<ActionResult>>,
}

impl ActionWrapper {
    pub fn new(action: GenericAction) -> Self {
        Self {
            action,
            completition_tx: None,
        }
    }

    pub fn notify(mut self, sender: Sender<ActionResult>) -> Self {
        self.completition_tx = Some(sender);
        self
    }

    pub fn action(&self) -> &GenericAction {
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
