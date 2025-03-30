use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::char,
    combinator::{map, map_res, opt},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use palette::{FromColor, Hsl, Hsluv, Srgb};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_hex(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        match Color::parse_color(input.trim()) {
            Ok((_, srgb)) => Ok(srgb),
            Err(_) => Err(format!("Failed to parse color: {}", input)),
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let (r, g, b, a) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                (r, g, b, 255)
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
                (r, g, b, a)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                (r, g, b, a)
            }
            _ => return None,
        };

        Some(Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }

    pub fn from_hsl(h: f32, s: f32, l: f32, a: Option<f32>) -> Self {
        let hsl = Hsl::new(h, s, l);
        let srgb: Srgb = Srgb::from_color(hsl);
        Self {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
            a: a.unwrap_or(1.0),
        }
    }

    pub fn from_hsluv(h: f32, s: f32, l: f32, a: Option<f32>) -> Self {
        let hsluv = Hsluv::new(h, s, l);
        let srgb: Srgb = Srgb::from_color(hsluv);
        Self {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
            a: a.unwrap_or(1.0),
        }
    }

    fn parse_hex(input: &str) -> IResult<&str, Color> {
        map_res(
            preceded(tag("#"), take_while1(|c: char| c.is_ascii_hexdigit())),
            |hex| Color::from_hex(hex).ok_or("Invalid hex color"),
        )(input)
    }

    fn parse_hsl(input: &str) -> IResult<&str, Color> {
        map(
            tuple((
                tag("hsl("),
                separated_pair(Color::parse_number, char(','), Color::parse_number),
                char(','),
                Color::parse_number,
                opt(preceded(char(','), Color::parse_number)),
                char(')'),
            )),
            |(_, (h, s), _, l, a, _)| Color::from_hsl(h, s, l, a),
        )(input)
    }

    fn parse_hsluv(input: &str) -> IResult<&str, Color> {
        map(
            tuple((
                tag("hsluv("),
                separated_pair(Color::parse_number, char(','), Color::parse_number),
                char(','),
                Color::parse_number,
                opt(preceded(char(','), Color::parse_number)),
                char(')'),
            )),
            |(_, (h, s), _, l, a, _)| Color::from_hsluv(h, s, l, a),
        )(input)
    }

    fn parse_number(input: &str) -> IResult<&str, f32> {
        map_res(is_not(",)"), |s: &str| f32::from_str(s.trim()))(input)
    }

    fn parse_color(input: &str) -> IResult<&str, Color> {
        alt((Color::parse_hex, Color::parse_hsluv, Color::parse_hsl))(input)
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

#[cfg(feature = "iced")]
impl From<Color> for iced_core::Color {
    fn from(val: Color) -> Self {
        iced_core::Color {
            r: val.r,
            g: val.g,
            b: val.b,
            a: val.a,
        }
    }
}

#[cfg(feature = "iced")]
impl From<Color> for iced_core::Background {
    fn from(val: Color) -> Self {
        Self::Color(iced_core::Color {
            r: val.r,
            g: val.g,
            b: val.b,
            a: val.a,
        })
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::parse(s)
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

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = String::serialize(&String::from(*self), serializer)?;
        Ok(s)
    }
}

#[cfg(feature = "serde")]
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
        assert_eq!(Color::parse(black), Ok(Color::new(0.0, 0.0, 0.0, 1.0)));

        let white = "#ffffff";
        assert_eq!(Color::parse(white), Ok(Color::new(1.0, 1.0, 1.0, 1.0)));

        let red = "#ff0000";
        assert_eq!(Color::parse(red), Ok(Color::new(1.0, 0.0, 0.0, 1.0)));

        let green = "#00ff00";
        assert_eq!(Color::parse(green), Ok(Color::new(0.0, 1.0, 0.0, 1.0)));

        let blue = "#0000ff";
        assert_eq!(Color::parse(blue), Ok(Color::new(0.0, 0.0, 1.0, 1.0)));
    }

    #[test]
    fn parse_rrggbbaa() {
        let black = "#00000000";
        assert_eq!(Color::parse(black), Ok(Color::new(0.0, 0.0, 0.0, 0.0)));

        let white = "#ffffffff";
        assert_eq!(Color::parse(white), Ok(Color::new(1.0, 1.0, 1.0, 1.0)));

        let red = "#ff0000ff";
        assert_eq!(Color::parse(red), Ok(Color::new(1.0, 0.0, 0.0, 1.0)));

        let green = "#00ff00ff";
        assert_eq!(Color::parse(green), Ok(Color::new(0.0, 1.0, 0.0, 1.0)));

        let blue = "#0000ffff";
        assert_eq!(Color::parse(blue), Ok(Color::new(0.0, 0.0, 1.0, 1.0)));
    }

    #[test]
    fn test_parse_hsl() {
        let color = Color::parse("hsl(180, 0.5, 0.4)").unwrap();
        assert!((color.r - 0.2).abs() < 1e-6);
        assert!((color.g - 0.6).abs() < 1e-6);
        assert!((color.b - 0.6).abs() < 1e-6);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_parse_hsluv() {
        let color_hsluv = Color::parse("hsluv(240, 80, 70)").unwrap();
        let color_hex = Color::parse("#60b2ef").unwrap();
        assert!((color_hsluv.r - color_hex.r).abs() < 1e-2);
        assert!((color_hsluv.g - color_hex.g).abs() < 1e-2);
        assert!((color_hsluv.b - color_hex.b).abs() < 1e-2);
        assert_eq!(color_hsluv.a, 1.0);
    }

    #[test]
    fn test_parse_invalid_color() {
        let result = Color::parse("invalidcolor");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_trim() {
        let color = Color::parse("  #ff00ff  ").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 1.0);
        assert_eq!(color.a, 1.0);
    }
}
