use std::sync::Arc;

use iced::{
    border::Radius,
    widget::{self, column, container::Style, Container},
    Border, Color, Element, Shadow, Vector,
};

use crate::{
    notification::Notification,
    theming::{self, Theme},
};

pub fn notification<'a, Message: 'a>(
    notification: Notification,
    theme: Option<&'a Theme>,
) -> Element<'a, Message> {
    let fallback = &theming::FALLBACK;
    let theme = &theme.unwrap_or(fallback).theme.notification;

    Container::new(widget::text(notification.text))
        .padding(8.0)
        .style(move |_| Style {
            background: Some(theme.background.into()),
            border: Border {
                radius: Radius::new(theme.radius),
                color: theme.border_color.into(),
                width: theme.border_width,
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(theme.shadow_x, theme.shadow_y),
                blur_radius: theme.shadow_blur,
            },
            text_color: Some(theme.text.into()),
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

    let fallback = &theming::FALLBACK;
    let theme = &theme.unwrap_or(fallback).theme.notification_list;
    column(entries)
        .spacing(theme.spacing)
        .padding(theme.padding)
        .into()
}
