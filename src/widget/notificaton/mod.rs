use std::sync::Arc;

use iced::{
    border::Radius,
    widget::{self, column, container::Style, Container},
    Border, Color, Element, Shadow, Theme, Vector,
};

use crate::notification::Notification;

pub fn notification<'a, Message: 'a>(notification: Notification) -> Element<'a, Message> {
    Container::new(widget::text(notification.text))
        .padding(8.0)
        .style(|theme: &Theme| Style {
            background: Some(theme.palette().background.into()),
            border: Border {
                radius: Radius::new(4.0),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(4.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .into()
}

pub fn notification_list<'a, Message: 'a>(
    notifications: &Vec<Arc<Notification>>,
) -> Element<'a, Message> {
    let mut entries = Vec::new();

    for i in notifications {
        let notify = notification((**i).clone());
        entries.push(notify);
    }

    column(entries).spacing(8.0).into()
}
