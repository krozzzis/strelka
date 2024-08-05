use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if !hex.starts_with('#') {
            return Err("Invalid hex color format. Expected '#rrggbb' or '#rrggbbaa'.".to_string());
        }

        match hex.len() {
            // #rrggbbaa
            9 => {
                let r = u8::from_str_radix(&hex[1..3], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[3..5], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[5..7], 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[7..9], 16).map_err(|e| e.to_string())?;

                Ok(Color {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    a: a as f32 / 255.0,
                })
            }

            // #rrggbb
            7 => {
                let r = u8::from_str_radix(&hex[1..3], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[3..5], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[5..7], 16).map_err(|e| e.to_string())?;

                Ok(Color {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    a: 1.0,
                })
            }

            _ => Err("Invalid hex color format. Expected '#rrggbb' or '#rrggbbaa'.".to_string()),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

impl From<Color> for iced::Color {
    fn from(val: Color) -> Self {
        iced::Color {
            r: val.r,
            g: val.g,
            b: val.b,
            a: val.a,
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::from_hex(s)
    }
}

impl From<Color> for String {
    fn from(val: Color) -> Self {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (val.r * 255.0) as u8,
            (val.g * 255.0) as u8,
            (val.b * 255.0) as u8,
            (val.a * 255.0) as u8
        )
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = String::serialize(&String::from(*self), serializer)?;
        Ok(s)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_hex_rrggbb() {
        let black = "#000000";
        assert_eq!(Color::from_hex(black), Ok(Color::new(0.0, 0.0, 0.0, 1.0)));

        let white = "#ffffff";
        assert_eq!(Color::from_hex(white), Ok(Color::new(1.0, 1.0, 1.0, 1.0)));

        let red = "#ff0000";
        assert_eq!(Color::from_hex(red), Ok(Color::new(1.0, 0.0, 0.0, 1.0)));

        let green = "#00ff00";
        assert_eq!(Color::from_hex(green), Ok(Color::new(0.0, 1.0, 0.0, 1.0)));

        let blue = "#0000ff";
        assert_eq!(Color::from_hex(blue), Ok(Color::new(0.0, 0.0, 1.0, 1.0)));
    }

    #[test]
    fn from_hex_rrggbbaa() {
        let black = "#00000000";
        assert_eq!(Color::from_hex(black), Ok(Color::new(0.0, 0.0, 0.0, 0.0)));

        let white = "#ffffffff";
        assert_eq!(Color::from_hex(white), Ok(Color::new(1.0, 1.0, 1.0, 1.0)));

        let red = "#ff0000ff";
        assert_eq!(Color::from_hex(red), Ok(Color::new(1.0, 0.0, 0.0, 1.0)));

        let green = "#00ff00ff";
        assert_eq!(Color::from_hex(green), Ok(Color::new(0.0, 1.0, 0.0, 1.0)));

        let blue = "#0000ffff";
        assert_eq!(Color::from_hex(blue), Ok(Color::new(0.0, 0.0, 1.0, 1.0)));
    }
}
