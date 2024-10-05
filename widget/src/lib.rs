pub mod buffer;
pub mod button;
pub mod container;
pub mod editor;
pub mod file_explorer;
pub mod list;
pub mod pane;
pub mod tab;
pub mod util;

use std::borrow::Cow;

pub type Label = Cow<'static, str>;
