use core::{
    buffer::{Buffer, FormattedBuffer},
    pane::{Pane, PaneId, VisiblePaneModel},
};

use action::{Action, ActionResult, ActionTransport, IntoAction, PaneAction};
use iced::{
    widget::{
        center, column, horizontal_space, mouse_area, row, stack, svg, text::IntoFragment, Column,
        MouseArea, Row, Space, Text,
    },
    Alignment, Border, Element, Length, Padding,
};
use theming::{theme, Theme};
use tokio::sync::mpsc;

use crate::{
    buffer::buffer,
    button::{a, icon_button},
    container::{background, background2},
    icon::Icon,
    pane::new_document::{self, new_document_pane},
    tab::{tab_bar, Tab},
    Label,
};

lazy_static::lazy_static! {
    pub static ref BUFFER: Buffer = Buffer::new("Hello\nAboba");
    pub static ref FORMATTED: FormattedBuffer = FormattedBuffer::from_buffer(&BUFFER);
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenPane(PaneId),
    ClosePane(PaneId),
    NewPane(Pane),
    NewDocument(new_document::Message),
    None,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Action(Action),
    Close,
    Maximize,
    Collapse,
    Drag,
    None,
}

pub fn pane_stack<'a>(model: VisiblePaneModel) -> Element<'a, Message, Theme> {
    let mut tabs: Vec<Tab<Message>> = model
        .panes
        .iter()
        .map(|(id, pane)| {
            let title: Option<Label> = match pane {
                Pane::Empty => None,
                Pane::NewDocument => Some("New tab".into()),
                Pane::Editor(_doc_id) => Some("File tab".into()),
                Pane::Buffer => Some("Buffer tab (EXPERIMENTAL)".into()),
                Pane::Config => Some("Config viewer".into()),
            };

            Tab {
                label: title,
                icon: None,
                selected: Some(*id) == model.opened,
                on_click: Some(Message::OpenPane(*id)),
                on_close: Some(Message::ClosePane(*id)),
                on_middle_click: Some(Message::ClosePane(*id)),
            }
        })
        .collect();

    let new_tab_button = Tab {
        label: None,
        icon: Some(svg::Handle::from_path("./images/plus.svg")),
        selected: false,
        on_click: Some(Message::NewPane(Pane::NewDocument)),
        on_close: None,
        on_middle_click: None,
    };

    tabs.push(new_tab_button);

    let tab_bar = tab_bar(tabs);

    let pane = if let Some(id) = model.opened {
        if let Some((_id, pane)) = model.panes.iter().find(|(x, _pane)| *x == id) {
            match pane {
                Pane::Empty => background(Space::new(Length::Fill, Length::Fill)).into(),
                Pane::NewDocument => new_document_pane().map(Message::NewDocument),
                Pane::Editor(_id) => background(center("file buffer")).into(),
                Pane::Buffer => background(buffer(&FORMATTED)).into(),
                Pane::Config => background(center("config buffer")).into(),
            }
        } else {
            Space::new(Length::Fill, Length::Fill).into()
        }
    } else {
        Space::new(Length::Fill, Length::Fill).into()
    };

    column![tab_bar, pane,].into()
}

pub fn top_bar<'a>() -> Element<'a, Msg, Theme> {
    let menu_button: Element<Msg, Theme> = icon_button(Icon::Menu).on_press(Msg::None).into();
    let add_button: Element<Msg, Theme> = icon_button(Icon::Add)
        .on_press(Msg::Action(
            PaneAction::Add(Pane::NewDocument).into_action(),
        ))
        .into();
    let close_button: Element<Msg, Theme> = icon_button(Icon::Close).on_press(Msg::Close).into();
    let maximize_button: Element<Msg, Theme> =
        icon_button(Icon::Maximize).on_press(Msg::Maximize).into();
    let collapse_button: Element<Msg, Theme> =
        icon_button(Icon::Collapse).on_press(Msg::Collapse).into();

    let topbar = row![
        menu_button,
        add_button,
        horizontal_space(),
        collapse_button,
        maximize_button,
        close_button
    ]
    .spacing(8.0)
    .padding(8.0)
    .width(Length::Fill);

    let bg =
        mouse_area(background2(Space::new(Length::Fill, Length::Fixed(52.0)))).on_press(Msg::Drag);

    stack![bg, topbar].into()
}

pub fn simplified_pane_tab<'a>(
    title: impl IntoFragment<'a>,
    id: PaneId,
) -> Element<'a, Msg, Theme> {
    let btn: Element<'a, Msg, Theme> = a::Button::new(
        Row::with_children(vec![
            Icon::File.svg().width(36.0).height(36.0).into(),
            Text::new(title)
                .size(20)
                .height(Length::Fill)
                .align_y(Alignment::Center)
                .align_x(Alignment::Start)
                .into(),
            horizontal_space().into(),
            icon_button(Icon::More).on_press(Msg::None).into(),
        ])
        .padding(8.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(theme!(tab.height))
    .padding(0)
    .on_press(Msg::Action(PaneAction::Open(id).into_action()))
    .into();

    MouseArea::new(btn)
        .on_middle_press(Msg::Action(PaneAction::Close(id).into_action()))
        .into()
}

pub fn simplified_pane_stack<'a>(
    brocker_tx: mpsc::Sender<ActionTransport>,
) -> Element<'a, Msg, Theme> {
    let topbar = top_bar();

    let pane_model: Option<Box<VisiblePaneModel>> = 'pane_model: {
        let (action, rx) = PaneAction::GetModel().into_transport_receive();
        if brocker_tx.blocking_send(action).is_err() {
            break 'pane_model None;
        }

        if let Ok(ActionResult::Value(value)) = rx.blocking_recv() {
            // Convert Option<Box<Option<VisiblePaneModel>>> -> Option<Box<VisiblePaneModel>>
            value
                .downcast()
                .ok()
                .and_then(|boxed: Box<Option<VisiblePaneModel>>| (*boxed).map(Box::new))
        } else {
            None
        }
    };

    let sidebar_tabs: Element<'a, Msg, Theme> = if let Some(model) = pane_model {
        let mut panes = Vec::new();
        for pane in model.panes {
            panes.push(pane);
        }

        let pane_tabs = panes.iter().map(|pane| {
            let id = pane.0;

            simplified_pane_tab(pane.1.title(), id)
        });

        Column::with_children(pane_tabs).spacing(8.0).into()
    } else {
        Text::new("Can't load pane model").into()
    };

    let notestack_text: Element<'a, Msg, Theme> = Text::new("Note stack")
        .color(iced::Color::from_rgb8(67, 67, 67))
        .size(20.0)
        .align_x(Alignment::Center)
        .height(theme!(tab.height))
        .width(Length::Fill)
        .into();
    let sidebar: Element<Msg, Theme> =
        background2(column![sidebar_tabs, notestack_text].spacing(8.0))
            .padding(Padding::new(8.0).top(0.0))
            .width(270)
            .height(Length::Fill)
            .into();

    let content: Element<Msg, Theme> = stack![
        background2(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
        background(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme| {
                iced::widget::container::Style {
                    text_color: Some(theme.generic.text.into()),
                    background: Some(theme.generic.background.into()),
                    border: Border {
                        radius: iced::border::Radius {
                            top_left: 18.0,
                            top_right: 0.0,
                            bottom_right: 0.0,
                            bottom_left: 0.0,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }),
    ]
    .into();

    column![topbar, row![sidebar, content]].into()
}
