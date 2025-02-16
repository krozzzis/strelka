use core::ThemeId;
use std::sync::Arc;

use theming::Theme;
use tokio::sync::mpsc;

use crate::{Action, IntoAction, Receiver};

#[derive(Debug, Clone)]
pub enum ThemeAction {
    MakeIndex,
    SetTheme(ThemeId),
    GetCurrentTheme(mpsc::Sender<Theme>),
}

impl IntoAction for ThemeAction {
    fn into_action(self) -> Action {
        Action {
            receiver: Receiver::Theme,
            content: Arc::new(self),
        }
    }
}
