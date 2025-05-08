use std::fs::read_to_string;
use strelka_core::{DataTree, Node, NodeDeserialize, Value};

pub const APPLICATION_CONFIG_NODE: &'static str = "application";

#[derive(Debug)]
pub struct ApplicationConfig {
    pub scale_factor: f32,
    pub decorations: bool,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            decorations: true,
        }
    }
}

impl NodeDeserialize for ApplicationConfig {
    fn deserialize_from_node(node: &Node) -> Result<Self, Box<dyn std::error::Error>> {
        let node = if node.get_name() == APPLICATION_CONFIG_NODE {
            node
        } else {
            node.get_child(APPLICATION_CONFIG_NODE)
                .ok_or("Cannot get app config node")?
        };

        let data_tree = DataTree::with_root(node.clone());

        let default = ApplicationConfig::default();

        let scale_factor = data_tree
            .get_value("scale_factor")
            .unwrap_or(&Value::Float(default.scale_factor))
            .as_float_or_default(default.scale_factor);

        let decorations = data_tree
            .get_value("decorations")
            .unwrap_or(&Value::Boolean(default.decorations))
            .as_boolean_or_default(default.decorations);

        Ok(ApplicationConfig {
            scale_factor,
            decorations,
        })
    }
}

pub fn load_config() -> ApplicationConfig {
    if let Ok(config_text) = read_to_string("./strelka.kdl") {
        let config = DataTree::from_text(&config_text).unwrap_or_default();
        let app_config =
            ApplicationConfig::deserialize_from_node(config.root()).unwrap_or_default();

        app_config
    } else {
        ApplicationConfig::default()
    }
}
