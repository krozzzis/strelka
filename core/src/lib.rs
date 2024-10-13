pub mod action;
pub mod buffer;
pub mod color;
pub mod document;
pub mod hotkey;
pub mod pane;
pub mod value;

pub use color::*;
pub use hotkey::*;
pub use smol_str;

pub type ThemeID = smol_str::SmolStr;
