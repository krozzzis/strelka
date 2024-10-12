mod border;
pub mod catalog;
mod font;
#[cfg(feature = "iced")]
pub mod iced;
pub mod index;
mod margin;
pub mod metadata;
mod padding;
mod styles;
mod theme;

use std::sync::{Arc, RwLock};

pub use border::*;
pub use core::color::*;
pub use font::*;
pub use margin::*;
pub use padding::*;
pub use theme::{Theme, FALLBACK};

lazy_static::lazy_static! {
    pub static ref THEME: Arc<RwLock<Theme>> = Arc::new(RwLock::new(FALLBACK));
}

#[macro_export]
macro_rules! theme {
    ($($field:ident).+) => {
        theming::THEME
            .read()
            .map(|theme| theme.$($field).+)
            .unwrap_or_else(|_| {
                log::warn!("Using fallback option");
                theming::FALLBACK.$($field).+
            })
    };
}
