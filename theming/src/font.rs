use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Font {
    pub family: Cow<'static, str>,
    #[serde(default)]
    pub weight: Weight,
    #[serde(default)]
    pub style: Style,
    #[serde(default)]
    pub size: f32,
}

impl Font {
    pub const SANS_SERIF: Font = Font {
        family: Cow::Borrowed("Sans Serif"),
        weight: Weight::Normal,
        style: Style::Normal,
        size: 20.0,
    };
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Sans Serif"),
            weight: Weight::Normal,
            style: Style::Normal,
            size: 20.0,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Weight {
    Light,
    #[default]
    Normal,
    SemiBold,
    Bold,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Style {
    #[default]
    Normal,
    Italic,
}
