use std::borrow::Cow;

use iced::{widget::svg::Handle, widget::svg::Svg};
use theming::Theme;

pub static MENU_ICON: &[u8] = include_bytes!("../../contrib/menu.svg");
pub static ADD_ICON: &[u8] = include_bytes!("../../contrib/add.svg");
pub static CLOSE_ICON: &[u8] = include_bytes!("../../contrib/close.svg");
pub static FILE_ICON: &[u8] = include_bytes!("../../contrib/file.svg");
pub static MAXIMIZE_ICON: &[u8] = include_bytes!("../../contrib/maximize.svg");
pub static COLLAPSE_ICON: &[u8] = include_bytes!("../../contrib/collapse.svg");
pub static MORE_ICON: &[u8] = include_bytes!("../../contrib/more.svg");

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Menu,
    Add,
    Close,
    File,
    Maximize,
    Collapse,
    More,
}

impl Icon {
    pub fn handle(&self) -> Handle {
        Handle::from_memory(Cow::Borrowed(match self {
            Icon::Menu => MENU_ICON,
            Icon::Add => ADD_ICON,
            Icon::Close => CLOSE_ICON,
            Icon::File => FILE_ICON,
            Icon::Maximize => MAXIMIZE_ICON,
            Icon::Collapse => COLLAPSE_ICON,
            Icon::More => MORE_ICON,
        }))
    }

    pub fn svg<'a>(&self) -> Svg<'a, Theme> {
        Svg::new(self.handle())
    }
}
