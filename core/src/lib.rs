pub mod buffer;
pub mod color;
pub mod command;
pub mod document;
pub mod hotkey;
#[cfg(feature = "iced")]
pub mod iced;
pub mod message;
pub mod pane;
pub mod theme;
pub mod value;

pub use color::*;
pub use hotkey::*;
pub use message::*;
pub use theme::*;

pub use smol_str;

pub type ThemeId = smol_str::SmolStr;
