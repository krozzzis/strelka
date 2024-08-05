use crate::theming::color::Color;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Theme {
    button: Button,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Button {
    hover: ButtonStyle,
    active: ButtonStyle,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    text: Color,
    background: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let text = toml::to_string(&theme);
        assert_eq!(
            text,
            Ok("[button.hover]\ntext = \"#000000FF\"\nbackground = \"#7F7F7FFF\"\n\n[button.active]\ntext = \"#000000FF\"\nbackground = \"#FFFFFFFF\"\n".to_string())
        );
    }
}
