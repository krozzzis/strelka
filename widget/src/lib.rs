use std::borrow::Cow;

pub mod button;
pub mod containers;
pub mod editor;
pub mod file_explorer;
pub mod list;
pub mod notificaton;
pub mod pane;
pub mod tabs;

pub type Label = Cow<'static, str>;
