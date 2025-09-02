use std::{collections::HashMap, sync::{Arc, RwLock}};

use smol_str::SmolStr;
pub struct Core {
    documents: Arc<RwLock<HashMap<SmolStr, Arc<Document>>>>,
}

impl Core {
    pub fn new() -> Self {
        Self { documents: Arc::new(RwLock::new(HashMap::new()))}
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    id: SmolStr,
    title: SmolStr,
    content: SmolStr,
}

impl Document {
    pub fn new(id: impl Into<SmolStr>, title: impl Into<SmolStr>, content: impl Into<SmolStr>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        }
    }

    pub fn id(&self) -> &SmolStr {
        &self.id
    }

    pub fn title(&self) -> &SmolStr {
        &self.title
    }

    pub fn content(&self) -> &SmolStr {
        &self.content
    }
}