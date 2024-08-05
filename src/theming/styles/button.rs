use crate::theming::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub hover: ButtonStyle,
    pub active: ButtonStyle,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    pub text: Color,
    pub background: Color,
}

impl Button {
    pub const FALLBACK: Button = Button {
        hover: ButtonStyle {
            text: Color::BLACK,
            background: Color::new(0.8, 0.8, 0.8, 1.0),
        },
        active: ButtonStyle {
            text: Color::BLACK,
            background: Color::new(0.9, 0.9, 0.9, 1.0),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Theme {
        pub button: Button,
    }

    #[test]
    fn serialize() {
        let theme = Theme {
            button: Button {
                active: ButtonStyle {
                    text: Color::new(0.0, 0.0, 0.0, 1.0),
                    background: Color::new(1.0, 1.0, 1.0, 1.0),
                },
                hover: ButtonStyle {
                    text: Color::new(0.0, 0.0, 0.0, 1.0),
                    background: Color::new(0.5, 0.5, 0.5, 1.0),
                },
            },
        };

        assert_eq!(
            toml::to_string(&theme),
            Ok("[button.hover]\ntext = \"#000000FF\"\nbackground = \"#7F7F7FFF\"\n\n[button.active]\ntext = \"#000000FF\"\nbackground = \"#FFFFFFFF\"\n".to_string())
        );
    }
}
