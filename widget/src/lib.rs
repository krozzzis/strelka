pub mod button;
pub mod container;
pub mod editor;
pub mod file_explorer;
pub mod list;
pub mod notificaton;
pub mod pane;
pub mod tabs;

use std::borrow::Cow;

pub type Label = Cow<'static, str>;
