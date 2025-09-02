use strelka_core::Document;
use smol_str::SmolStr;
use std::sync::Arc;

use iced::widget::{center, text, column, container};
use iced::{Element, Task};

pub struct DocumentView {
    pub id: SmolStr,
    pub title: SmolStr,
    pub content_preview: SmolStr,
}

impl DocumentView {
    pub fn render<'a, T: 'a>(&'a self) -> iced::Element<'a, T> {
        container(
            column![
                text(self.title.as_str()),
                text(self.content_preview.as_str()),
            ]
        )
        .style(container::rounded_box)
        .padding(12.0)
        .into()
    }
}

impl From<Arc<Document>> for DocumentView {
    fn from(doc: Arc<Document>) -> Self {
        Self {
            id: doc.id().clone(),
            title: doc.title().clone(),
            content_preview: doc.content().clone(),
        }
    }
}