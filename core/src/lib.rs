pub mod buffer;
pub mod color;
pub mod document;
pub mod hotkey;
pub mod pane;
pub mod value;
pub mod message;
pub mod theme;

pub use color::*;
pub use hotkey::*;
pub use message::*;
pub use theme::Theme;

pub use smol_str;

pub type ThemeId = smol_str::SmolStr;
