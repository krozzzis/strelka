use core::ThemeId;

use theming::Theme;
use tokio::sync::mpsc;

use crate::{Action, IntoAction};

#[derive(Debug, Clone)]
pub enum ThemeAction {
    MakeIndex,
    SetTheme(ThemeId),
    GetCurrentTheme(mpsc::Sender<Theme>),
}

impl IntoAction for ThemeAction {
    fn into_action(self) -> Action {
        Action::Theme(self)
    }
}
