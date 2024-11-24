use std::borrow::Cow;

use iced::{widget::svg::Handle, widget::svg::Svg};
use theming::Theme;

pub static MENU_ICON: &[u8] = include_bytes!("../../contrib/menu.svg");
pub static ADD_ICON: &[u8] = include_bytes!("../../contrib/add.svg");

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Menu,
    Add,
}

impl Icon {
    pub fn handle(&self) -> Handle {
        Handle::from_memory(Cow::Borrowed(match self {
            Icon::Menu => MENU_ICON,
            Icon::Add => ADD_ICON,
        }))
    }

    pub fn svg<'a>(&self) -> Svg<'a, Theme> {
        Svg::new(self.handle())
    }
}
