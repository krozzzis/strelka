use core::{smol_str::SmolStr, value::Value, Color};
use std::{collections::HashMap, path::PathBuf};

use iced::daemon::{Appearance, DefaultStyle};

#[derive(Debug, Default, Clone)]
pub struct Theme {
    stylesheet: HashMap<SmolStr, Value>,
}

impl Theme {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: impl Into<SmolStr>, value: impl Into<Value>) {
        self.stylesheet.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &SmolStr) -> Option<&Value> {
        self.stylesheet.get(key)
    }
    pub fn get_integer_or_default(&self, key: &SmolStr, default: i32) -> i32 {
        self.get(key).and_then(Value::as_integer).unwrap_or(default)
    }

    pub fn get_float_or_default(&self, key: &SmolStr, default: f32) -> f32 {
        self.get(key).and_then(Value::as_float).unwrap_or(default)
    }

    pub fn get_boolean_or_default(&self, key: &SmolStr, default: bool) -> bool {
        self.get(key).and_then(Value::as_boolean).unwrap_or(default)
    }

    pub fn get_color_or_default(&self, key: &SmolStr, default: Color) -> Color {
        self.get(key).and_then(Value::as_color).unwrap_or(default)
    }

    pub fn get_path_or_default<'a>(&'a self, key: &SmolStr, default: &'a PathBuf) -> &'a PathBuf {
        self.get(key).and_then(Value::as_path).unwrap_or(default)
    }

    pub fn get_string_or_default<'a>(&'a self, key: &SmolStr, default: &'a SmolStr) -> &'a SmolStr {
        self.get(key).and_then(Value::as_string).unwrap_or(default)
    }
}

#[cfg(feature = "iced")]
impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.get_color_or_default(&SmolStr::new_static("background.color"), Color::WHITE).into(),
            text_color: self.get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK).into(),
        }
    }
}
