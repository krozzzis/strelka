pub mod catalog;
mod color;
#[cfg(feature = "iced")]
pub mod iced;
pub mod metadata;
mod styles;
mod theme;

use std::sync::{Arc, RwLock};

pub use color::*;
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
            .unwrap_or(theming::FALLBACK.$($field).+)
    };
}
