use config::Config;
use core::{
    action::{
        DocumentAction, DocumentActionResponse, FileAction, FileActionResponse, GenericAction,
        PaneAction,
    },
    document::{DocumentHandler, DocumentStore},
    pane::{Pane, PaneModel},
    value::Value,
    ThemeID,
};
use crossbeam::channel::{bounded, Receiver, Sender};
use iced::widget::text_editor::Content;
use log::info;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use theming::{catalog::Catalog, index::ThemeIndex, Theme};

pub struct State {
    pub documents: DocumentStore<Content>,
    pub panes: PaneModel,
    pub themes: Catalog,
    pub config: Config,
}

impl State {
    pub async fn make_theme_index(&mut self) {
        let index = ThemeIndex::load_from_directory("./themes/").await;
        if let Ok(index) = index {
            self.themes.set_index(index);
        }
    }

    pub fn get_theme(&self) -> Arc<Theme> {
        self.themes.get_current_theme()
    }

    pub async fn set_theme(&mut self, id: ThemeID) {
        info!("Set theme {id}");
        if let Ok(mut theme) = theming::THEME.write() {
            *theme = (*self.get_theme()).clone();
        }
        self.config
            .insert("system", "theme", Value::String(id.clone()));
        self.themes.set_theme(id).await;
    }
}

pub struct ActionBrocker {
    receiver: Receiver<GenericAction>,
    document_sender: Option<Sender<DocumentAction>>,
}

impl ActionBrocker {
    pub fn new(rx: Receiver<GenericAction>) -> Self {
        Self {
            receiver: rx,
            document_sender: None,
        }
    }

    pub fn document(mut self, document_tx: Sender<DocumentAction>) -> Self {
        self.document_sender = Some(document_tx);
        self
    }

    pub async fn run(&mut self) {
        while let Ok(action) = self.receiver.recv() {
            info!("Brocker. Processing: {action:?}");
            match action {
                GenericAction::File(_) => todo!(),
                GenericAction::Pane(_) => todo!(),
                GenericAction::Document(action) => {
                    if let Some(tx) = &self.document_sender {
                        let _ = tx.send(action);
                    }
                }
                GenericAction::Theme(_) => todo!(),
            }
        }
    }
}

pub struct DocumentActor {
    documents: DocumentStore<Content>,
    receiver: Receiver<DocumentAction>,
    brocker_sender: Sender<GenericAction>,
}

impl DocumentActor {
    pub fn new(rx: Receiver<DocumentAction>, brocker_tx: Sender<GenericAction>) -> Self {
        Self {
            documents: DocumentStore::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        while let Ok(action) = self.receiver.recv() {
            match action {
                DocumentAction::Add(handler) => {
                    let content = Content::with_text(&handler.text_content);
                    let handler = DocumentHandler {
                        text_content: content,
                        path: handler.path.clone(),
                        filename: handler.filename.clone(),
                        changed: handler.changed,
                    };
                    self.documents.add(handler);
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(id);
                    let message = GenericAction::Pane(PaneAction::Add(pane));
                    let _ = self.brocker_sender.send(message);
                }
                DocumentAction::Save(_id) => {
                    todo!()
                }
                DocumentAction::Remove(id) => {
                    self.documents.remove(&id);
                }
            }
        }
    }
}

pub struct FileActor {
    receiver: Receiver<FileAction>,
    brocker_sender: Sender<GenericAction>,
}

impl FileActor {
    pub fn new(rx: Receiver<FileAction>, brocker_tx: Sender<GenericAction>) -> Self {
        Self {
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        while let Ok(action) = self.receiver.recv() {
            match action {
                FileAction::PickFile(tx) => {
                    if let Ok((path, content)) = pick_file(None).await {
                        let handler = DocumentHandler {
                            text_content: content,
                            path: path.clone(),
                            filename: get_file_name(&path),
                            changed: false,
                        };

                        let (tx, rx) = bounded(1);
                        let _ =
                            self.brocker_sender
                                .send(GenericAction::Document(DocumentAction::Add(
                                    Arc::new(handler),
                                    tx,
                                )));
                        if let DocumentActionResponse::DocumentAdded(doc_id) = rx.recv().unwrap() {
                            let pane = Pane::Editor(doc_id);

                            // If opened pane is NewDocument, replace it with Editor pane
                            // otherwise add new one with Editor
                            if let Some(&Pane::NewDocument) = self.state.panes.get_open() {
                                self.state.panes.replace(
                                    &self.state.panes.get_open_id().cloned().unwrap_or(0usize),
                                    pane,
                                );
                            } else {
                                let pane_id = self.state.panes.add(pane);
                                self.state.panes.open(&pane_id);
                            }
                        }
                    }
                }
                FileAction::OpenFileCurrentTab(_path) => {
                    todo!()
                }
                FileAction::OpenFileForceCurrentTab(_path) => {
                    todo!()
                }
                FileAction::OpenFileNewTab(_path) => {
                    todo!()
                }
            }
        }
    }
}

pub async fn open_file(path: impl AsRef<Path>) -> Result<(PathBuf, String), ()> {
    let path = path.as_ref();
    let content = tokio::fs::read_to_string(path).await.map_err(|_| ())?;
    Ok((path.to_owned(), content))
}

pub async fn pick_file(directory: Option<PathBuf>) -> Result<(PathBuf, String), ()> {
    let handler = if let Some(dir) = directory {
        rfd::AsyncFileDialog::new().set_directory(dir)
    } else {
        rfd::AsyncFileDialog::new()
    }
    .pick_file()
    .await;

    if let Some(path) = handler {
        let content = open_file(path.path()).await.map_err(|_| ())?;
        Ok(content)
    } else {
        Err(())
    }
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_owned()
}
