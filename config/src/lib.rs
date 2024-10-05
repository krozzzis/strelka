use core::value::Value;
use std::collections::HashMap;

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, Default, Clone)]
pub struct Config {
    #[serde(flatten)]
    namespaces: HashMap<String, HashMap<String, Value>>,
}


impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, namespace: impl Into<String>, property: impl Into<String>, value: impl Into<Value>) {
        let namespace = namespace.into();
        if let Some(namespace) = self.namespaces.get_mut(&namespace) {
            namespace.insert(property.into(), value.into());
        } else {
            let mut ns = HashMap::new();
            ns.insert(property.into(), value.into());
            self.namespaces.insert(namespace, ns);
        }
    }

    pub fn remove_property(&mut self, namespace: impl Into<String>, property: impl Into<String>) -> Option<Value> {
        if let Some(namespace) = self.namespaces.get_mut(&namespace.into()) {
            namespace.remove(&property.into())
        } else {
            None
        }
    }

    pub fn get(&self, namespace: impl Into<String>, property: impl Into<String>) -> Option<Value> {
        let namespace: String = namespace.into();
        if let Some(namespace) = self.namespaces.get(&namespace) {
            namespace.get(&property.into()).cloned()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{value::Value, Color};

    use crate::Config;

    #[test]
    fn test2() {
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
        assert_eq!(config.get("system", "accent"), Some(Value::Color(Color::WHITE)));
        assert_eq!(config.get("system", "name"), Some(Value::String(String::from("Strelka"))));
        assert_eq!(config.get("system", "debug"), Some(Value::Boolean(false)));
    }
}

