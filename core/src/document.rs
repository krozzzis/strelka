use std::{collections::HashMap, path::PathBuf};

pub type DocumentId = usize;

#[derive(Default, Debug)]
pub struct DocumentHandler<Content> {
    pub text_content: Content,
    pub path: PathBuf,
    pub filename: String,
    pub changed: bool,
}

#[derive(Default, Debug)]
pub struct DocumentStore<Content> {
    documents: HashMap<DocumentId, DocumentHandler<Content>>,
    next_id: DocumentId,
}

impl<Content> DocumentStore<Content> {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, document: DocumentHandler<Content>) -> DocumentId {
        let id = self.next_id;

        self.documents.insert(id, document);
        self.next_id += 1;

        id
    }

    pub fn remove(&mut self, id: &DocumentId) -> Option<DocumentHandler<Content>> {
        self.documents.remove(id)
    }

    pub fn count(&self) -> usize {
        self.documents.len()
    }

    pub fn get(&self, id: &DocumentId) -> Option<&DocumentHandler<Content>> {
        self.documents.get(id)
    }

    pub fn get_mut(&mut self, id: &DocumentId) -> Option<&mut DocumentHandler<Content>> {
        self.documents.get_mut(id)
    }
}
