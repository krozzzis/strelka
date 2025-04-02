use crate::smol_str::SmolStr;
use crate::value::Value;
use crate::Color;
use std::collections::HashMap;
use std::fmt::Debug;

/// Theme trait that defines the interface for stylesheets
pub trait Theme: Debug + Send + Sync {
    /// Get a color value from the theme
    fn get_color(&self, key: &SmolStr) -> Option<Color>;

    /// Get a color value with a default fallback
    fn get_color_or_default(&self, key: &SmolStr, default: Color) -> Color {
        self.get_color(key).unwrap_or(default)
    }

    /// Get a float value from the theme
    fn get_float(&self, key: &SmolStr) -> Option<f32>;

    /// Get a float value with a default fallback
    fn get_float_or_default(&self, key: &SmolStr, default: f32) -> f32 {
        self.get_float(key).unwrap_or(default)
    }

    /// Get a string value from the theme
    fn get_string(&self, key: &SmolStr) -> Option<SmolStr>;

    /// Get a string value with a default fallback
    fn get_string_or_default(&self, key: &SmolStr, default: SmolStr) -> SmolStr {
        self.get_string(key).unwrap_or(default)
    }

    /// Get a boolean value from the theme
    fn get_bool(&self, key: &SmolStr) -> Option<bool>;

    /// Get a boolean value with a default fallback
    fn get_bool_or_default(&self, key: &SmolStr, default: bool) -> bool {
        self.get_bool(key).unwrap_or(default)
    }

    fn get_style_properties(&self, path: &str) -> HashMap<String, Value>;
}

pub trait StyleConverter: Default {
    fn from_theme(theme: &impl Theme, path: &str) -> Self;
}
