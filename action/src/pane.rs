use core::pane::{Pane, PaneId, VisiblePaneModel};

use tokio::sync::mpsc;

use crate::{Action, IntoAction};

#[derive(Debug, Clone)]
pub enum PaneAction {
    Close(PaneId),
    Open(PaneId),
    Add(Pane, Option<mpsc::Sender<PaneId>>),
    Replace(PaneId, Pane),
    GetOpen(mpsc::Sender<Option<Pane>>),
    GetOpenId(mpsc::Sender<Option<PaneId>>),
    GetModel(mpsc::Sender<Option<VisiblePaneModel>>),
}

impl IntoAction for PaneAction {
    fn into_action(self) -> Action {
        Action::Pane(self)
    }
}
