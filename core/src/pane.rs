use std::collections::HashMap;

use crate::document::DocumentId;

pub type PaneId = usize;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Pane {
    #[default]
    Empty,
    NewDocument,
    Editor(DocumentId),
    Buffer,
}

#[derive(Default, Debug)]
pub struct PaneModel {
    panes: HashMap<PaneId, Pane>,
    visible: Vec<PaneId>,
    next_id: PaneId,
    open: Option<PaneId>,
}

impl PaneModel {
    pub fn new() -> Self {
        Self {
            panes: HashMap::new(),
            visible: Vec::new(),
            next_id: 1,
            open: None,
        }
    }

    pub fn add(&mut self, pane: Pane) -> PaneId {
        let id = self.next_id;

        self.panes.insert(id, pane);
        self.visible.push(id);
        self.next_id += 1;

        id
    }

    pub fn remove(&mut self, id: &PaneId) -> Option<Pane> {
        // If removing pane is opened, close them
        if self.open == Some(*id) {
            // if there are more panels, open the last one
            if let Some(last) = self.visible.last() {
                self.open = Some(*last);
            } else {
                // Otherwise leave it not opened
                self.open = None;
            }
        }

        self.visible.retain(|&x| x != *id);
        self.panes.remove(id)
    }

    pub fn contains(&self, id: &PaneId) -> bool {
        self.panes.contains_key(id)
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
        self.visible
            .iter()
            .filter_map(|id| self.panes.get(id).map(|pane| (id, pane)))
            .collect()
    }

    pub fn open(&mut self, id: &PaneId) {
        if self.panes.contains_key(id) && self.open != Some(*id) {
            self.open = Some(*id);
        }
    }

    pub fn get_open_id(&self) -> Option<&PaneId> {
        self.open.as_ref()
    }

    pub fn get_open(&self) -> Option<&Pane> {
        if let Some(id) = self.open {
            self.panes.get(&id)
        } else {
            None
        }
    }

    pub fn replace(&mut self, id: &PaneId, new: Pane) {
        if let Some(pane) = self.panes.get_mut(id) {
            *pane = new;
        }
    }
}

#[derive(Debug, Clone)]
pub enum PaneAction {
    Close(PaneId),
    Open(PaneId),
    Add(Pane),
    Replace(PaneId, Pane),
}
