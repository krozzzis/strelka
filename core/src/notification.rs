use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Notification {
    pub text: String,
    pub kind: NotificationKind,
}

impl Notification {
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: NotificationKind::None,
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: NotificationKind::Error,
        }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: NotificationKind::Info,
        }
    }

    pub fn warn(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: NotificationKind::Warning,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NotificationKind {
    Info,
    Error,
    Warning,
    None,
}

#[derive(Default)]
pub struct NotificationList {
    pub last_id: usize,
    pub notifications: HashMap<usize, Arc<Notification>>,
}

impl NotificationList {
    pub fn new() -> Self {
        Self::default()
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
