use core::value::Value;
use std::{collections::HashMap, path::PathBuf};

use core::smol_str::SmolStr;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    #[cfg_attr(feature = "serde", serde(flatten))]
    namespaces: HashMap<SmolStr, HashMap<SmolStr, Value>>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        namespace: impl Into<SmolStr>,
        property: impl Into<SmolStr>,
        value: impl Into<Value>,
    ) {
        let namespace = namespace.into();
        if let Some(namespace) = self.namespaces.get_mut(&namespace) {
            namespace.insert(property.into(), value.into());
        } else {
            let mut ns = HashMap::new();
            ns.insert(property.into(), value.into());
            self.namespaces.insert(namespace, ns);
        }
    }

    pub fn remove_property(
        &mut self,
        namespace: impl Into<SmolStr>,
        property: impl Into<SmolStr>,
    ) -> Option<Value> {
        if let Some(namespace) = self.namespaces.get_mut(&namespace.into()) {
            namespace.remove(&property.into())
        } else {
            None
        }
    }

    pub fn get(
        &self,
        namespace: impl Into<SmolStr>,
        property: impl Into<SmolStr>,
    ) -> Option<Value> {
        let namespace = namespace.into();
        if let Some(namespace) = self.namespaces.get(&namespace) {
            namespace.get(&property.into()).cloned()
        } else {
            None
        }
    }

    /// Returns an iterator over all properties in the configuration.
    ///
    /// The iterator yields tuples in the form `(&SmolStr, &SmolStr, &Value)`, where:
    /// - The first element is a reference to the namespace name.
    /// - The second element is a reference to the property name within the namespace.
    /// - The third element is a reference to the property value.
    pub fn properties(&self) -> impl Iterator<Item = (&SmolStr, &SmolStr, &Value)> {
        self.namespaces.iter().flat_map(|(namespace, properties)| {
            properties
                .iter()
                .map(move |(property_name, value)| (namespace, property_name, value))
        })
    }

    /// Merges another `Config` into this one, replacing existing values with those from the other.
    pub fn merge(&mut self, other: Config) {
        for (namespace, properties) in other.namespaces {
            // Get a mutable reference to the namespace entry, or insert an empty HashMap if it doesn't exist
            let entry = self.namespaces.entry(namespace).or_default();

            // Merge each property in the current namespace
            for (property_name, value) in properties {
                entry.insert(property_name, value); // Insert the new property, overwriting if it exists
            }
        }
    }

    /// Merges another `Config` into this one without replacing existing values.
    pub fn populate(&mut self, other: Config) {
        for (namespace, properties) in other.namespaces {
            let entry = self.namespaces.entry(namespace).or_default();

            for (property_name, value) in properties {
                entry.entry(property_name).or_insert(value);
            }
        }
    }

    #[cfg(feature = "serde")]
    pub fn load(path: &PathBuf) -> Result<Config, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config = toml::from_str(&text).map_err(|e| e.to_string())?;
        Ok(config)
    }

    #[cfg(feature = "serde")]
    pub fn load_or_create_default(path: &PathBuf, default: Config) -> Result<Config, String> {
        if path.is_file() && std::fs::exists(path).is_ok() {
            let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            let config = toml::from_str(&text).map_err(|e| e.to_string())?;
            Ok(config)
        } else {
            let text = toml::to_string(&default).map_err(|e| e.to_string())?;
            std::fs::write(path, text).map_err(|e| e.to_string())?;
            Ok(default)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{value::Value, Color};

    use core::smol_str::SmolStr;

    use super::Config;

    #[test]
    fn deserialization() {
        let text = "
            [system]
            scale = 2.0
            version = 128
            accent = \"#ffffff\"
            name = \"Strelka\"
            debug = false
        ";
        let config: Config = toml::from_str(text).unwrap();
        assert_eq!(config.get("system", "scale"), Some(Value::Float(2.0)));
        assert_eq!(config.get("system", "version"), Some(Value::Integer(128)));
        assert_eq!(
            config.get("system", "accent"),
            Some(Value::Color(Color::WHITE))
        );
        assert_eq!(
            config.get("system", "name"),
            Some(Value::String(SmolStr::new("Strelka")))
        );
        assert_eq!(config.get("system", "debug"), Some(Value::Boolean(false)));
    }
}
