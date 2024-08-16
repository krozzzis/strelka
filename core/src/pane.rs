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
    prev_open: Option<PaneId>,
}

impl PaneModel {
    pub fn new() -> Self {
        Self {
            panes: HashMap::new(),
            next_id: 1,
            open: None,
            prev_open: None,
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
            if self.contains(&self.prev_open.unwrap_or(0)) {
                // Open previously opened pane if exists yet
                self.open = self.prev_open;
            } else if self.count() > 0 {
                // if there are more panels, open the last one
                let mut panes = self.list();
                panes.sort_unstable_by(|a, b| a.0.cmp(b.0));
                if let Some((newest, _pane)) = panes.last() {
                    self.open = Some(**newest);
                }
            } else {
                // Otherwise leave it not opened
                self.open = None;
            }
        }

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
        self.panes.iter().collect()
    }

    pub fn open(&mut self, id: &PaneId) {
        if self.panes.contains_key(id) && self.open != Some(*id) {
            self.prev_open = self.open;
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
}
