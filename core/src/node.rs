use std::collections::HashMap;

use crate::{smol_str::SmolStr, Value};
use kdl::{KdlDocument, KdlNode, KdlValue};

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

pub fn parse_kdl_document(document: &KdlDocument) -> Result<Node, String> {
    let mut root = Node::new();
    for node in document.nodes() {
        let name = node.name().value();

        let mut child = parse_kdl_node(node)?;
        child.name = SmolStr::new(name);

        if document.nodes().len() == 1 {
            root = child;
        } else {
            root.add_child(child);
        }
    }
    Ok(root)
}

fn parse_kdl_value(value: &KdlValue) -> Value {
    match value {
        KdlValue::String(value) => Value::String(value.into()),
        KdlValue::Integer(value) => Value::Integer(*value as i32),
        KdlValue::Float(value) => Value::Float(*value as f32),
        KdlValue::Bool(value) => Value::Boolean(*value),
        KdlValue::Null => Value::String(String::from("Null").into()),
    }
}

fn parse_kdl_node(node: &KdlNode) -> Result<Node, String> {
    let mut root = Node::new();
    for entry in node.entries() {
        if let Some(key) = entry.name() {
            let prop_name: SmolStr = key.value().into();
            let value = parse_kdl_value(entry.value());
            root.set_property(prop_name, value);
        } else if root.get_value().is_none() {
            let value = parse_kdl_value(entry.value());
            root.set_value(value);
        }
    }

    for doc in node.children() {
        for node in doc.nodes() {
            let name = node.name().value();

            let mut child = parse_kdl_node(node)?;
            child.name = SmolStr::new(name);

            root.add_child(child);
        }
    }
    Ok(root)
}

pub trait NodeDeserialize: Sized {
    fn deserialize_from_node(node: &Node) -> Result<Self, Box<dyn std::error::Error>>;
}
