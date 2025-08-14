use std::{path::PathBuf, str::FromStr};

use smol_str::SmolStr;

use crate::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Color(Color),
    Path(PathBuf),
    String(SmolStr),
}

impl Value {
    pub fn as_integer(&self) -> Option<i32> {
        if let Value::Integer(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_integer_or_default(&self, default: i32) -> i32 {
        self.as_integer().unwrap_or(default)
    }

    pub fn as_float(&self) -> Option<f32> {
        if let Value::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_float_or_default(&self, default: f32) -> f32 {
        self.as_float().unwrap_or(default)
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let Value::Boolean(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_boolean_or_default(&self, default: bool) -> bool {
        self.as_boolean().unwrap_or(default)
    }

    pub fn as_color(&self) -> Option<Color> {
        if let Value::Color(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_color_or_default(&self, default: Color) -> Color {
        self.as_color().unwrap_or(default)
    }

    pub fn as_path(&self) -> Option<&PathBuf> {
        if let Value::Path(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_path_or_default<'a>(&'a self, default: &'a PathBuf) -> &'a PathBuf {
        self.as_path().unwrap_or(default)
    }

    pub fn as_string(&self) -> Option<&SmolStr> {
        if let Value::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_string_or_default<'a>(&'a self, default: &'a SmolStr) -> &'a SmolStr {
        self.as_string().unwrap_or(default)
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Value::Color(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl serde::de::Visitor<'_> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer, float, boolean, string, or color")
            }

            fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
                Ok(Value::Integer(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::Float(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Boolean(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                if let Ok(color) = Color::from_str(value) {
                    Ok(Value::Color(color))
                } else {
                    Ok(Value::String(SmolStr::new(value)))
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                if let Ok(color) = Color::from_str(&value) {
                    Ok(Value::Color(color))
                } else {
                    Ok(Value::String(SmolStr::new(value)))
                }
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}


