use std::collections::HashMap;

use crate::{smol_str::SmolStr, Value};
use kdl::{KdlDocument, KdlNode, KdlValue};

#[derive(Debug, Clone, Default)]
pub struct Node {
    name: SmolStr,
    properties: HashMap<SmolStr, Value>,
    children: HashMap<SmolStr, Node>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    pub fn set_property(&mut self, key: SmolStr, value: Value) {
        self.properties.insert(key, value);
    }

    pub fn get_properties(&self) -> &HashMap<SmolStr, Value> {
        &self.properties
    }

    pub fn get_child(&self, key: &str) -> Option<&Node> {
        self.children.get(key)
    }

    pub fn add_child(&mut self, key: SmolStr, node: Node) {
        self.children.insert(key, node);
    }

    pub fn get_or_create_child(&mut self, key: &str) -> &mut Node {
        if !self.children.contains_key(key) {
            self.children.insert(SmolStr::new(key), Node::new());
        }
        self.children.get_mut(key).unwrap()
    }

    pub fn get_value(&self, path: &str) -> Option<&Value> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        // Check if the last part is a property or a child node with a default value
        let property_name = parts.last().unwrap();
        let mut current_node = self;

        // Navigate to the node that should contain our property
        for i in 0..parts.len() - 1 {
            current_node = current_node.get_child(parts[i])?;
        }

        // First try to get it as a direct property
        if let Some(value) = current_node.get_property(property_name) {
            return Some(value);
        }

        // If not found as property, check if it's a child node with a default value
        if let Some(child_node) = current_node.get_child(property_name) {
            // Try to get "value0" as the default property
            if let Some(value) = child_node.get_property("value0") {
                return Some(value);
            }
        }

        None
    }
}

pub fn parse_kdl_document(document: &KdlDocument) -> Result<Node, String> {
    let mut root = Node::new();
    for node in document.nodes() {
        let name = node.name().value().to_string();
        println!("{name}");

        let child = parse_kdl_node(node)?;

        if document.nodes().len() == 1 {
            root = child;
        } else {
            root.add_child(name.into(), child);
        }
    }
    Ok(root)
}

pub fn parse_kdl_node(node: &KdlNode) -> Result<Node, String> {
    let mut root = Node::new();
    for entry in node.entries() {
        println!("{entry:?}");
        if let Some(key) = entry.name() {
            let prop_name: SmolStr = key.value().into();
            let value = match entry.value() {
                KdlValue::String(value) => Value::String(value.into()),
                KdlValue::Integer(value) => Value::Integer(*value as i32),
                KdlValue::Float(value) => Value::Float(*value as f32),
                KdlValue::Bool(value) => Value::Boolean(*value),
                KdlValue::Null => Value::String(String::from("Null").into()),
            };
            println!("{prop_name} - {value:?}");
            root.set_property(prop_name.into(), value);
        }
    }
    for doc in node.children() {
        let new_node = parse_kdl_document(doc)?;
        // root.add_child(new_node);
    }
    Ok(root)
}
