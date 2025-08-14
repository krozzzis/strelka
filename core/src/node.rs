use std::collections::HashMap;

use crate::{smol_str::SmolStr, Value};

#[derive(Debug, Clone, Default)]
pub struct Node {
    name: SmolStr,
    value: Option<Value>,
    properties: HashMap<SmolStr, Value>,
    children: HashMap<SmolStr, Node>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
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

    pub fn add_child(&mut self, node: Node) {
        self.children.insert(node.name.clone(), node);
    }

    pub fn get_or_create_child(&mut self, key: &str) -> &mut Node {
        if !self.children.contains_key(key) {
            self.children.insert(SmolStr::new(key), Node::new());
        }
        self.children.get_mut(key).unwrap()
    }

    pub fn get_name(&self) -> &SmolStr {
        &self.name
    }
}

pub trait NodeDeserialize: Sized {
    fn deserialize_from_node(node: &Node) -> Result<Self, Box<dyn std::error::Error>>;
}
