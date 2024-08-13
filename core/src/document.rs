use std::{path::PathBuf, sync::Arc};

pub type DocumentId = usize;

pub struct DocumentHandler<Content> {
    pub text_content: Content,
    pub path: PathBuf,
    pub filename: Arc<String>,
    pub changed: bool,
}
