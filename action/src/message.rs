use crate::{Action, IntoAction};

#[derive(Debug, Clone)]
pub struct Message {
    pub destination: String,
    pub kind: String,
    pub payload: Option<String>,
}

impl IntoAction for Message {
    fn into_action(self) -> Action {
        Action::Message(self)
    }
}
