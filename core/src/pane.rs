use std::collections::HashMap;

use crate::document::DocumentId;

pub type PaneId = usize;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Pane {
    #[default]
    Empty,
    NewDocument,
    Editor(DocumentId),
}

#[derive(Default, Debug)]
pub struct PaneModel {
    panes: HashMap<PaneId, Pane>,
    next_id: PaneId,
    open: Option<PaneId>,
}

impl PaneModel {
    pub fn new() -> Self {
        Self {
            panes: HashMap::new(),
            next_id: 1,
            open: None,
        }
    }

    pub fn add(&mut self, pane: Pane) -> PaneId {
        let id = self.next_id;

        self.panes.insert(id, pane);
        self.next_id += 1;

        id
    }

    pub fn remove(&mut self, id: &PaneId) -> Option<Pane> {
        // If removing pane is opened, close them
        if self.open == Some(*id) {
            self.open = None
        }

        self.panes.remove(id)
    }

    pub fn count(&self) -> usize {
        self.panes.len()
    }

    pub fn get(&self, id: &PaneId) -> Option<&Pane> {
        self.panes.get(id)
    }

    pub fn get_mut(&mut self, id: &PaneId) -> Option<&mut Pane> {
        self.panes.get_mut(id)
    }

    pub fn list(&self) -> Vec<(&PaneId, &Pane)> {
        self.panes.iter().collect()
    }

    pub fn open(&mut self, id: &PaneId) {
        if self.panes.contains_key(id) {
            self.open = Some(*id);
        } else {
            self.open = None;
        }
    }

    pub fn get_open(&self) -> Option<&PaneId> {
        self.open.as_ref()
    }
}
