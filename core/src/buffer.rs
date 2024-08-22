use crate::Color;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub text: String,
}

impl Buffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.text.lines()
    }
}

#[derive(Debug, Clone)]
pub struct FormattedBuffer {
    pub elements: Vec<Element>,
}

impl FormattedBuffer {
    pub fn from_buffer(buffer: &Buffer) -> Self {
        let elements = buffer
            .lines()
            .map(|line| {
                Element::Span(
                    line.to_string(),
                    SpanAttributes {
                        font_size: 16.0,
                        font_color: Color::BLACK,
                    },
                )
            })
            .collect();

        Self { elements }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Element> {
        self.elements.iter()
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Span(String, SpanAttributes),
}

#[derive(Debug, Clone)]
pub struct SpanAttributes {
    pub font_size: f32,
    pub font_color: Color,
}
