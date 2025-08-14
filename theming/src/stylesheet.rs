use std::{collections::HashMap, path::Path};
use strelka_core::{smol_str::SmolStr, theme::StyleConverter, Color, Theme, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleNode {
    properties: HashMap<SmolStr, Value>,
    children: HashMap<SmolStr, StyleNode>,
}

impl StyleNode {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    pub fn get_properties(&self) -> &HashMap<SmolStr, Value> {
        &self.properties
    }

    pub fn child(&self, name: &str) -> Option<&StyleNode> {
        self.children.get(name)
    }

    pub fn set_property(&mut self, key: SmolStr, value: Value) {
        self.properties.insert(key, value);
    }

    pub fn add_child(&mut self, name: SmolStr, node: StyleNode) {
        self.children.insert(name, node);
    }

    pub fn get_or_create_child(&mut self, name: &str) -> &mut StyleNode {
        if !self.children.contains_key(name) {
            self.children.insert(SmolStr::new(name), StyleNode::new());
        }
        self.children.get_mut(name).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleSheet {
    root: StyleNode,
}

impl StyleSheet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn root(&self) -> &StyleNode {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut StyleNode {
        &mut self.root
    }

    pub fn get_value(&self, path: &str) -> Option<&Value> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let property_name = parts.last().unwrap();
        let mut current_node = &self.root;

        for part in parts.iter().take(parts.len() - 1) {
            current_node = current_node.child(part)?;
        }

        if let Some(value) = current_node.property(property_name) {
            return Some(value);
        }

        if let Some(child_node) = current_node.child(property_name) {
            if let Some(value) = child_node.property("value0") {
                return Some(value);
            }
        }

        None
    }

    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = smol::fs::read_to_string(path)
            .await
            .map_err(|e| e.to_string())?;
        
        let stylesheet: StyleSheet = ron::from_str(&content)
            .map_err(|e| format!("Failed to parse RON: {}", e))?;

        Ok(stylesheet)
    }

    pub fn get_node(&self, path: &str) -> Option<&StyleNode> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.root;

        for part in parts {
            if let Some(node) = current.child(part) {
                current = node;
            } else {
                return None;
            }
        }

        Some(current)
    }
}

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub background: Color,
    pub text_color: Color,
    pub border_radius: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            background: Color::new(0.2, 0.2, 0.2, 1.0),
            text_color: Color::WHITE,
            border_radius: 4.0,
        }
    }
}

impl StyleConverter for ButtonStyle {
    fn from_theme(theme: &Theme, path: &str) -> Self {
        let mut style = ButtonStyle::default();

        if let Some(background) = theme
            .inner
            .get_color(&SmolStr::new(format!("{path}.background")))
        {
            style.background = background;
        }
        
        if let Some(text_color) = theme.inner.get_color(&SmolStr::new(format!("{path}.text"))) {
            style.text_color = text_color;
        }

        if let Some(radius) = theme
            .inner
            .get_float(&SmolStr::new(format!("{path}.border_radius")))
        {
            style.border_radius = radius;
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strelka_core::Color;

    #[test]
    fn test_ron_theme_parsing() {
        let ron_content = concat!(
            "(\n",
            "    root: (\n",
            "        properties: {},\n",
            "        children: {\n",
            "            \"button\": (\n",
            "                properties: {},\n",
            "                children: {\n",
            "                    \"active\": (\n",
            "                        properties: {\n",
            "                            \"background\": \"#191919\",\n",
            "                            \"text_color\": \"#dddddd\",\n",
            "                            \"border_radius\": 10.0,\n",
            "                        },\n",
            "                        children: {},\n",
            "                    ),\n",
            "                },\n",
            "            ),\n",
            "        },\n",
            "    ),\n",
            ")"
        );

        let stylesheet: StyleSheet = ron::from_str(ron_content).expect("Failed to parse RON");
        
        // Test that we can retrieve values
        let background = stylesheet.get_value("button.active.background");
        assert!(background.is_some());
        
        if let Some(Value::Color(color)) = background {
            assert_eq!(*color, Color::from_hex("191919").unwrap());
        } else {
            panic!("Expected color value");
        }
        
        let border_radius = stylesheet.get_value("button.active.border_radius");
        assert!(border_radius.is_some());
        
        if let Some(Value::Float(radius)) = border_radius {
            assert_eq!(*radius, 10.0);
        } else {
            panic!("Expected float value");
        }
    }
}
