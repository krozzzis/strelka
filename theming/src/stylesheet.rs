use std::{collections::HashMap, path::Path};
use strelka_core::{smol_str::SmolStr, theme::StyleConverter, value::Value, Color, Theme};

use kdl::{KdlDocument, KdlNode, KdlValue};

#[derive(Debug, Clone, Default)]
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

    // Получение или создание дочернего узла
    pub fn get_or_create_child(&mut self, name: &str) -> &mut StyleNode {
        if !self.children.contains_key(name) {
            self.children.insert(SmolStr::new(name), StyleNode::new());
        }
        self.children.get_mut(name).unwrap()
    }
}

#[derive(Debug, Clone, Default)]
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

        // Check if the last part is a property or a child node with a default value
        let property_name = parts.last().unwrap();
        let mut current_node = &self.root;

        // Navigate to the node that should contain our property
        for i in 0..parts.len() - 1 {
            current_node = current_node.child(parts[i])?;
        }

        // First try to get it as a direct property
        if let Some(value) = current_node.property(property_name) {
            return Some(value);
        }

        // If not found as property, check if it's a child node with a default value
        if let Some(child_node) = current_node.child(property_name) {
            // Try to get "value0" as the default property
            if let Some(value) = child_node.property("value0") {
                return Some(value);
            }
        }

        None
    }

    // Загрузка темы из KDL файла
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = smol::fs::read_to_string(path)
            .await
            .map_err(|e| e.to_string())?;
        let doc: KdlDocument = content.parse().map_err(|e: kdl::KdlError| e.to_string())?;

        let mut stylesheet = StyleSheet::new();
        stylesheet.parse_kdl_document(&doc)?;

        Ok(stylesheet)
    }

    // Разбор KDL документа в тему
    fn parse_kdl_document(&mut self, doc: &KdlDocument) -> Result<(), String> {
        for node in doc.nodes() {
            let name = node.name().value().to_string();
            let child_node = self.root_mut().get_or_create_child(&name);
            Self::parse_kdl_node(node, child_node)?;
        }
        Ok(())
    }

    fn parse_kdl_node(node: &KdlNode, style_node: &mut StyleNode) -> Result<(), String> {
        // Process named properties
        for entry in node.entries() {
            if let Some(name) = entry.name() {
                let prop_name = SmolStr::new(name.value());
                let prop_value = entry.value();
                let value = Self::parse_kdl_value(prop_value)?;
                style_node.set_property(prop_name, value);
            }
        }

        // Process unnamed values
        for (i, entry) in node.entries().iter().enumerate() {
            if entry.name().is_none() {
                let prop_name = SmolStr::new(format!("value{}", i));
                let value = Self::parse_kdl_value(entry.value())?;
                style_node.set_property(prop_name, value);
            }
        }

        // Process child nodes
        if let Some(children_doc) = node.children() {
            for child_node in children_doc.nodes() {
                let child_name = SmolStr::new(child_node.name().value());
                let child_style = style_node.get_or_create_child(&child_name);
                Self::parse_kdl_node(child_node, child_style)?;
            }
        }

        Ok(())
    }

    // Преобразование KDL значения в StyleValue
    fn parse_kdl_value(value: &KdlValue) -> Result<Value, String> {
        match value {
            KdlValue::String(s) => {
                if s.starts_with('#') {
                    return Self::parse_color(s);
                }
                Ok(Value::String(SmolStr::new(s)))
            }
            KdlValue::Float(n) => Ok(Value::Float(*n as f32)),
            KdlValue::Integer(n) => Ok(Value::Integer(*n as i32)),
            KdlValue::Bool(b) => Ok(Value::Boolean(*b)),
            KdlValue::Null => Err("Null values are not supported".to_string()),
        }
    }

    // Парсинг цвета из строки
    fn parse_color(color_str: &str) -> Result<Value, String> {
        if color_str.len() != 7 && color_str.len() != 9 {
            return Err(format!("Invalid color format: {}", color_str));
        }

        let r = u8::from_str_radix(&color_str[1..3], 16).map_err(|e| e.to_string())?;
        let g = u8::from_str_radix(&color_str[3..5], 16).map_err(|e| e.to_string())?;
        let b = u8::from_str_radix(&color_str[5..7], 16).map_err(|e| e.to_string())?;

        let a = if color_str.len() == 9 {
            u8::from_str_radix(&color_str[7..9], 16).map_err(|e| e.to_string())?
        } else {
            255
        };

        Ok(Value::Color(Color::new_hex(r, g, b, a)))
    }

    pub fn get_node(&self, path: &str) -> Option<&StyleNode> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.root;

        for part in parts {
            if let Some(node) = current.child(part) {
                current = node;
            } else {
                // Если узел не найден, используем текущий
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

        // Try to get background either as direct property or from child node
        if let Some(background) = theme
            .inner
            .get_color(&SmolStr::new(format!("{path}.background")))
        {
            style.background = background;
        }
        // Try to get text_color either as direct property or from child node
        if let Some(text_color) = theme.inner.get_color(&SmolStr::new(format!("{path}.text"))) {
            style.text_color = text_color;
        }

        // Try to get border_radius either as direct property or from child node
        if let Some(radius) = theme
            .inner
            .get_float(&SmolStr::new(format!("{path}.border_radius")))
        {
            style.border_radius = radius;
        }

        style
    }
}
