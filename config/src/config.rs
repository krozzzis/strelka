use std::path::PathBuf;
use strelka_core::smol_str::SmolStr;
use strelka_core::value::Value;
use strelka_core::Color;

use dashmap::mapref::one::Ref;
use dashmap::DashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Key {
    pub namespace: SmolStr,
    pub id: SmolStr,
}

#[derive(Debug, Clone)]
pub struct Config {
    data: DashMap<Key, Value>,
}

impl Config {
    pub fn get_value(&self, key: &Key) -> Option<Ref<Key, Value>> {
        self.data.get(key)
    }

    pub fn set_value(&mut self, key: Key, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get_integer(&self, key: &Key) -> Option<i32> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Integer(result) => Some(*result),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_float(&self, key: &Key) -> Option<f32> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Float(result) => Some(*result),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_boolean(&self, key: &Key) -> Option<bool> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Boolean(result) => Some(*result),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_color(&self, key: &Key) -> Option<Color> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Color(result) => Some(*result),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_path(&mut self, key: &Key) -> Option<PathBuf> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Path(result) => Some(result.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_string(&mut self, key: &Key) -> Option<SmolStr> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::String(result) => Some(result.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}
