use std::{collections::HashMap, path::PathBuf};
use strelka_core::{smol_str::SmolStr, Color, GenericTheme, Value};

// use iced::daemon::{Appearance, DefaultStyle};

use crate::stylesheet::StyleSheet;

#[derive(Debug, Default, Clone)]
pub struct Theme {
    stylesheet: StyleSheet,
}

impl Theme {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_stylesheet(stylesheet: StyleSheet) -> Self {
        Self { stylesheet }
    }

    pub fn get(&self, path: &str) -> Option<&Value> {
        self.stylesheet.get_value(path)
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

impl GenericTheme for Theme {
    fn get_color(&self, key: &SmolStr) -> Option<Color> {
        self.get(key.as_str()).and_then(Value::as_color)
    }

    fn get_float(&self, key: &SmolStr) -> Option<f32> {
        self.get(key.as_str()).and_then(Value::as_float)
    }

    fn get_string(&self, key: &SmolStr) -> Option<SmolStr> {
        self.get(key.as_str()).and_then(Value::as_string).cloned()
    }

    fn get_bool(&self, key: &SmolStr) -> Option<bool> {
        self.get(key.as_str()).and_then(Value::as_boolean)
    }

    fn get_style_properties(&self, path: &str) -> HashMap<String, Value> {
        let mut properties = HashMap::new();
        if let Some(node) = self.stylesheet.get_node(path) {
            for (key, value) in node.get_properties() {
                properties.insert(key.to_string(), value.clone());
            }
        }
        properties
    }
}
