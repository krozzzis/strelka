use std::sync::Arc;

use iced::{
    border::Radius,
    widget::{self, column, container::Style, Container},
    Border, Color, Element, Shadow, Vector,
};

use core::notification::Notification;
use theming::{theme, Theme};

pub fn notification<'a, Message: 'a>(notification: Notification) -> Element<'a, Message, Theme> {
    Container::new(widget::text(notification.text))
        .padding(8.0)
        .style(|theme: &Theme| Style {
            background: Some(theme.notification.background.into()),
            border: Border {
                radius: Radius::new(theme.notification.radius),
                color: theme.notification.border_color.into(),
                width: theme.notification.border_width,
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(theme.notification.shadow_x, theme.notification.shadow_y),
                blur_radius: theme.notification.shadow_blur,
            },
            text_color: Some(theme.notification.text.into()),
        })
        .into()
}

pub fn notification_list<'a, Message: 'a>(
    notifications: &Vec<Arc<Notification>>,
) -> Element<'a, Message, Theme> {
    let mut entries = Vec::new();

    for i in notifications {
        let notify = notification((**i).clone());
        entries.push(notify);
    }

    column(entries)
        .spacing(theme!(notification_list.spacing))
        .padding(theme!(notification_list.padding))
        .into()
}
