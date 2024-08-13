use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Notification {
    pub text: String,
    pub kind: NotificationKind,
}

#[derive(Debug, Clone, Copy)]
pub enum NotificationKind {
    Info,
    Error,
    Warning,
    None,
}

pub struct NotificationList {
    pub last_id: usize,
    pub notifications: HashMap<usize, Arc<Notification>>,
}

impl NotificationList {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            notifications: HashMap::new(),
        }
    }

    pub fn add(&mut self, notification: Arc<Notification>) -> usize {
        self.notifications.insert(self.last_id, notification);
        let output = self.last_id;
        self.last_id += 1;
        output
    }

    pub fn remove(&mut self, id: usize) -> Option<Arc<Notification>> {
        self.notifications.remove(&id)
    }

    pub fn to_vec(&self) -> Vec<Arc<Notification>> {
        let mut list: Vec<(&usize, &Arc<Notification>)> = self.notifications.iter().collect();
        list.sort_by(|a, b| a.0.cmp(b.0));
        list.iter().cloned().map(|(_, x)| x.clone()).collect()
    }
}
