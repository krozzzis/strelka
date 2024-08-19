use crate::{font::Font, Border, Color, Margin};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub hover: TabStyle,
    pub active: TabStyle,
    pub selected: TabStyle,
    pub height: f32,
    pub min_width: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TabStyle {
    pub text: Color,
    pub background: Color,
    pub margin: Margin,
    pub border: Border,
    #[serde(default)]
    pub font: Font,
}

impl Tab {
    pub const FALLBACK: Tab = Self {
        height: 40.0,
        min_width: 150.0,
        hover: TabStyle {
            text: Color::BLACK,
            background: Color::new(0.8, 0.8, 0.8, 1.0),
            margin: Margin::new(4.0),
            border: Border::with_radius(4.0),
            font: Font::SANS_SERIF,
        },
        active: TabStyle {
            text: Color::BLACK,
            background: Color::new(1.0, 1.0, 1.0, 1.0),
            margin: Margin::new(4.0),
            border: Border::with_radius(4.0),
            font: Font::SANS_SERIF,
        },
        selected: TabStyle {
            text: Color::BLACK,
            background: Color::new(0.9, 0.9, 0.9, 1.0),
            margin: Margin::new(4.0),
            border: Border::with_radius(4.0),
            font: Font::SANS_SERIF,
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Theme {
        pub tab: Tab,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize() {
        let theme = Theme { tab: Tab::FALLBACK };

        assert_eq!(toml::to_string(&theme), Ok("".to_string()));
    }
}
