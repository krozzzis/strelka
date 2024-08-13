use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationList {
    pub padding: f32,
    pub spacing: f32,
}

impl NotificationList {
    pub const FALLBACK: NotificationList = NotificationList {
        padding: 4.0,
        spacing: 4.0,
    };
}
