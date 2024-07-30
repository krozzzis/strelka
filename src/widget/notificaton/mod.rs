use std::sync::Arc;

use iced::{
    border::Radius,
    widget::{self, column, container::Style, Container},
    Border, Color, Element, Shadow, Vector,
};

use crate::{notification::Notification, theme::Theme};

pub fn notification<'a, Message: 'a>(
    notification: Notification,
    theme: Option<&'a Theme>,
) -> Element<'a, Message> {
    Container::new(widget::text(notification.text))
        .padding(8.0)
        .style(move |_| {
            let theme = theme.cloned().unwrap_or(Theme::default());
            Style {
                background: Some(theme.background2.into()),
                border: Border {
                    radius: Radius::new(theme.element_radius),
                    color: theme.border_color,
                    width: 1.0,
                },
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(4.0, 4.0),
                    blur_radius: 8.0,
                },
                text_color: Some(theme.text),
            }
        })
        .into()
}

pub fn notification_list<'a, Message: 'a>(
    notifications: &Vec<Arc<Notification>>,
    theme: Option<&'a Theme>,
) -> Element<'a, Message> {
    let mut entries = Vec::new();

    for i in notifications {
        let notify = notification((**i).clone(), theme);
        entries.push(notify);
    }

    column(entries).spacing(8.0).into()
}
