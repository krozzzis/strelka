use std::str::FromStr;

use smol_str::SmolStr;

use crate::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Color(Color),
    String(SmolStr),
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer, float, boolean, string, or color")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Integer(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
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
