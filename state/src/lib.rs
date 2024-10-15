mod action;

pub use action::{ActionResult, ActionWrapper};

use config::Config;
use core::{
    action::{DocumentAction, FileAction, GenericAction, PaneAction},
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
    receiver: Receiver<ActionWrapper>,
    document_sender: Option<Sender<DocumentAction>>,
    file_sender: Option<Sender<FileAction>>,
    pane_sender: Option<Sender<PaneAction>>,
}

impl ActionBrocker {
    pub fn new(rx: Receiver<ActionWrapper>) -> Self {
        Self {
            receiver: rx,
            document_sender: None,
            file_sender: None,
            pane_sender: None,
        }
    }

    pub fn document_actor(mut self, document_tx: Sender<DocumentAction>) -> Self {
        self.document_sender = Some(document_tx);
        self
    }

    pub fn file_actor(mut self, file_tx: Sender<FileAction>) -> Self {
        self.file_sender = Some(file_tx);
        self
    }

    pub fn pane_actor(mut self, pane_tx: Sender<PaneAction>) -> Self {
        self.pane_sender = Some(pane_tx);
        self
    }

    pub async fn run(&mut self) {
        info!("Started Brocker's thread");
        while let Ok(wrapper) = self.receiver.recv() {
            info!("Brocker. Processing: {wrapper:?}");
            let action = wrapper.action();
            match action {
                GenericAction::File(action) => {
                    if let Some(tx) = &self.file_sender {
                        let _ = tx.send(action.clone());
                    }
                }
                GenericAction::Pane(action) => {
                    if let Some(tx) = &self.pane_sender {
                        let _ = tx.send(action.clone());
                    }
                }
                GenericAction::Document(action) => {
                    if let Some(tx) = &self.document_sender {
                        let _ = tx.send(action.clone());
                    }
                }
                GenericAction::Theme(_) => todo!(),
            }
            wrapper.try_notify_complete(ActionResult::Success);
        }
    }
}

pub struct DocumentActor {
    documents: DocumentStore<Content>,
    receiver: Receiver<DocumentAction>,
    brocker_sender: Sender<ActionWrapper>,
}

impl DocumentActor {
    pub fn new(rx: Receiver<DocumentAction>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            documents: DocumentStore::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started DocumentActor's thread");
        while let Ok(action) = self.receiver.recv() {
            info!("Processing: {action:?}");
            match action {
                DocumentAction::Add(handler, tx) => {
                    let content = Content::with_text(&handler.text_content);
                    let handler = DocumentHandler {
                        text_content: content,
                        path: handler.path.clone(),
                        filename: handler.filename.clone(),
                        changed: handler.changed,
                    };
                    let doc_id = self.documents.add(handler);
                    if let Some(tx) = tx {
                        let _ = tx.send(doc_id);
                    }
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(id);
                    let message = GenericAction::Pane(PaneAction::Add(pane, None));
                    let _ = self.brocker_sender.send(ActionWrapper::new(message));
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
    brocker_sender: Sender<ActionWrapper>,
}

impl FileActor {
    pub fn new(rx: Receiver<FileAction>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started FileActor's thread");
        while let Ok(action) = self.receiver.recv() {
            info!("Processing: {action:?}");
            match action {
                FileAction::PickFile => {
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
                                .send(ActionWrapper::new(GenericAction::Document(
                                    DocumentAction::Add(Arc::new(handler), Some(tx)),
                                )));

                        if let Ok(doc_id) = rx.recv() {
                            let pane = Pane::Editor(doc_id);

                            // If opened pane is NewDocument, replace it with Editor pane
                            // otherwise add new one with Editor
                            let (tx, rx) = bounded(1);
                            let _ =
                                self.brocker_sender
                                    .send(ActionWrapper::new(GenericAction::Pane(
                                        PaneAction::GetOpen(tx),
                                    )));
                            if let Ok(Some(Pane::NewDocument)) = rx.recv() {
                                let (tx, rx) = bounded(1);
                                let _ = self.brocker_sender.send(ActionWrapper::new(
                                    GenericAction::Pane(PaneAction::GetOpenId(tx)),
                                ));

                                if let Ok(Some(opened_id)) = rx.recv() {
                                    let message =
                                        GenericAction::Pane(PaneAction::Replace(opened_id, pane));
                                    let _ = self.brocker_sender.send(ActionWrapper::new(message));
                                }
                            } else {
                                let (tx, rx) = bounded(1);
                                let _ = self.brocker_sender.send(ActionWrapper::new(
                                    GenericAction::Pane(PaneAction::Add(pane, Some(tx))),
                                ));
                                if let Ok(pane_id) = rx.recv() {
                                    let _ = self.brocker_sender.send(ActionWrapper::new(
                                        GenericAction::Pane(PaneAction::Open(pane_id)),
                                    ));
                                }
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

pub struct PaneActor {
    panes: PaneModel,
    receiver: Receiver<PaneAction>,
    brocker_sender: Sender<ActionWrapper>,
}

impl PaneActor {
    pub fn new(rx: Receiver<PaneAction>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            panes: PaneModel::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started PaneActors's thread");
        while let Ok(action) = self.receiver.recv() {
            info!("Processing: {action:?}");
            match action {
                PaneAction::Close(id) => {
                    let pane = self.panes.remove(&id);

                    // Close document if Editor pane was closed
                    if let Some(Pane::Editor(doc_id)) = pane {
                        let message = GenericAction::Document(DocumentAction::Remove(doc_id));
                        let _ = self.brocker_sender.send(ActionWrapper::new(message));
                    }

                    // If there no panes left, create a NewDocument one
                    if self.panes.count() == 0 {
                        let id = self.panes.add(Pane::NewDocument);
                        self.panes.open(&id);
                    }
                }
                PaneAction::Open(id) => self.panes.open(&id),
                PaneAction::Add(pane, tx) => {
                    let id = self.panes.add(pane);
                    self.panes.open(&id);
                    if let Some(tx) = tx {
                        let _ = tx.send(id);
                    }
                }
                PaneAction::Replace(id, pane) => {
                    self.panes.replace(&id, pane);
                }
                PaneAction::GetOpen(tx) => {
                    let opened = self.panes.get_open().cloned();
                    let _ = tx.send(opened);
                }
                PaneAction::GetOpenId(tx) => {
                    let opened_id = self.panes.get_open_id().cloned();
                    let _ = tx.send(opened_id);
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
