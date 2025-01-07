use core::value::Value;

use core::smol_str::SmolStr;
use core::Color;
use std::path::PathBuf;

use dashmap::mapref::one::Ref;
use dashmap::DashMap;

pub struct Config {
    data: DashMap<SmolStr, Value>,
}

impl Config {
    pub fn get_value(&self, key: &SmolStr) -> Option<Ref<SmolStr, Value>> {
        self.data.get(key)
    }

    pub fn set_value(&mut self, key: SmolStr, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get_integer(&self, key: &SmolStr) -> Option<i64> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Integer(result) => Some(*result),
                _ => None,
            }           
        } else {
            None
        }
    }

    pub fn get_float(&self, key: &SmolStr) -> Option<f64> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Float(result) => Some(*result),
                _ => None,
            }           
        } else {
            None
        }
    }

    pub fn get_boolean(&self, key: &SmolStr) -> Option<bool> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Boolean(result) => Some(*result),
                _ => None,
            }           
        } else {
            None
        }
    }

    pub fn get_color(&self, key: &SmolStr) -> Option<Color> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Color(result) => Some(*result),
                _ => None,
            }           
        } else {
            None
        }
    }

    pub fn get_path(&mut self, key: &SmolStr) -> Option<PathBuf> {
        if let Some(value) = self.get_value(key) {
            match value.value() {
                Value::Path(result) => Some(result.clone()),
                _ => None,
            }           
        } else {
            None
        }
    }
    
    pub fn get_string(&mut self, key: &SmolStr) -> Option<SmolStr> {
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
