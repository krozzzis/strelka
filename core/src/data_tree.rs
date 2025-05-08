use crate::node::{parse_kdl_document, Node};
use crate::smol_str::SmolStr;
use crate::Value;
use kdl::KdlDocument;

#[derive(Debug, Clone)]
pub struct DataTree {
    root: Node,
}

impl Default for DataTree {
    fn default() -> Self {
        Self::new()
    }
}

impl DataTree {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn with_root(root: Node) -> Self {
        Self { root }
    }

    pub fn from_text(text: &str) -> Result<Self, String> {
        let document: KdlDocument = text
            .parse()
            .map_err(|e| format!("Failed to parse KDL: {}", e))?;

        let root = parse_kdl_document(&document)?;

        Ok(Self { root })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    pub fn get_value(&self, path: &str) -> Option<&Value> {
        let parts: Vec<&str> = path.split('.').collect();
        self.get_value_by_parts(&parts)
    }

    fn get_value_by_parts(&self, parts: &[&str]) -> Option<&Value> {
        if parts.is_empty() {
            return None;
        }

        let mut current = &self.root;

        for (i, &part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                if let Some(prop) = current.get_property(part) {
                    return Some(prop);
                } else if let Some(child) = current.get_child(part) {
                    return child.get_value();
                }
                return None;
            } else if i == 0 {
                if current.get_name() != part {
                    return None;
                }
            } else if let Some(next) = current.get_child(part) {
                current = next;
            } else {
                return None;
            }
        }

        None
    }

    pub fn get_node(&self, path: &str) -> Option<&Node> {
        if path.is_empty() {
            return Some(&self.root);
        }

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.root;

        for &part in &parts {
            if let Some(child) = current.get_child(part) {
                current = child;
            } else {
                return None;
            }
        }

        Some(current)
    }

    pub fn get_or_create_node(&mut self, path: &str) -> &mut Node {
        if path.is_empty() {
            return &mut self.root;
        }

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &mut self.root;

        for &part in &parts {
            current = current.get_or_create_child(part);
        }

        current
    }

    pub fn set_value(&mut self, path: &str, value: Value) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err("Empty path".to_string());
        }

        let node_path = &parts[..parts.len() - 1];
        let value_key = parts[parts.len() - 1];

        let mut current = &mut self.root;

        for &part in node_path {
            current = current.get_or_create_child(part);
        }

        current.set_property(SmolStr::new(value_key), value);

        Ok(())
    }
}
