use crate::smol_str::SmolStr;
use crate::value::Value;
use crate::Color;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Theme trait that defines the interface for stylesheets
pub trait GenericTheme: Debug + Send + Sync {
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
    fn from_theme(theme: &Theme, path: &str) -> Self;
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub inner: Arc<dyn GenericTheme>,
}

#[derive(Debug, Clone)]
pub struct DefaultTheme {}

impl GenericTheme for DefaultTheme {
    fn get_color(&self, _key: &SmolStr) -> Option<Color> {
        None
    }

    fn get_float(&self, _key: &SmolStr) -> Option<f32> {
        None
    }

    fn get_string(&self, _key: &SmolStr) -> Option<SmolStr> {
        None
    }

    fn get_bool(&self, _key: &SmolStr) -> Option<bool> {
        None
    }

    fn get_style_properties(&self, _path: &str) -> HashMap<String, Value> {
        HashMap::new()
    }
}

#[cfg(feature = "iced")]
impl iced_core::theme::Base for Theme {
    fn base(&self) -> iced_core::theme::Style {
        iced_core::theme::Style {
            background_color: self
                .inner
                .get_color_or_default(&SmolStr::new_static("background.color"), Color::WHITE)
                .into(),
            text_color: self
                .inner
                .get_color_or_default(&SmolStr::new_static("text.color"), Color::BLACK)
                .into(),
        }
    }

    fn palette(&self) -> Option<iced_core::theme::Palette> {
        None
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            inner: Arc::new(DefaultTheme {}),
        }
    }
}
