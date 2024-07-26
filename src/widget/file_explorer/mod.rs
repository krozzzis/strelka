use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use iced::{
    border::Radius,
    widget::{button, component, row, stack, text, Button, Column, Component, Container},
    Background, Border, Color, Element, Length, Padding, Renderer, Theme,
};

pub struct FileExplorer<'a, Message> {
    pub files: Vec<&'a PathBuf>,
    pub dirs: Vec<&'a PathBuf>,
    pub on_click: Option<Box<dyn Fn(PathBuf) -> Message>>,
}

impl<'a, Message> FileExplorer<'a, Message> {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
            on_click: None,
        }
    }

    pub fn with_content(content: &'a [PathBuf]) -> Self {
        let files = content.iter().filter(|x| x.is_file()).collect();
        let dirs = content.iter().filter(|x| x.is_dir()).collect();
        Self {
            files,
            dirs,
            on_click: None,
        }
    }

    pub fn with_content_maybe(content: Option<&'a [PathBuf]>) -> Self {
        if let Some(content) = content {
            Self::with_content(content)
        } else {
            Self::new()
        }
    }

    pub fn file_click<F>(mut self, func: F) -> Self
    where
        F: 'static + Fn(PathBuf) -> Message,
    {
        self.on_click = Some(Box::new(func));
        self
    }
}

impl<'a, Msg> Component<Msg> for FileExplorer<'a, Msg> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Msg> {
        match event {
            Message::OpenDir(_path) => {}

            Message::OpenFile(path) => {
                if let Some(func) = &self.on_click {
                    return Some(func(path));
                }
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme, Renderer> {
        let dirs = self.dirs.iter().map(|path| {
            Container::new(dir_entry(path, Message::OpenDir((*path).clone())))
                .width(Length::Fill)
                .into()
        });

        let files = self.files.iter().map(|path| {
            Container::new(file_entry(path, Message::OpenFile((*path).clone())))
                .width(Length::Fill)
                .into()
        });

        Container::new(Column::from_iter(dirs.chain(files)))
            .padding(8.0)
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message> From<FileExplorer<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(widget: FileExplorer<'a, Message>) -> Self {
        component(widget)
    }
}

#[derive(Clone)]
pub enum Message {
    OpenFile(PathBuf),
    OpenDir(PathBuf),
}

fn file_entry<'a, Msg: Clone + 'a>(path: &'a Path, click: Msg) -> Element<'a, Msg> {
    let title = path
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("");

    Button::new(title)
        .on_press(click)
        .width(Length::Fill)
        .style(|_theme: &Theme, status| match status {
            button::Status::Active | button::Status::Disabled => button::Style {
                background: None,
                ..Default::default()
            },

            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(Background::Color(Color::new(0.85, 0.85, 0.85, 1.0))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(4.0),
                },
                ..Default::default()
            },
        })
        .padding(Padding::new(4.0).left(24.0))
        .into()
}

fn dir_entry<'a, Msg: Clone + 'a>(path: &'a Path, click: Msg) -> Element<'a, Msg> {
    let title = path
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("");

    stack([
        Button::new(title)
            .on_press(click)
            .width(Length::Fill)
            .style(|_theme: &Theme, status| match status {
                button::Status::Active | button::Status::Disabled => button::Style {
                    background: None,
                    ..Default::default()
                },

                button::Status::Hovered | button::Status::Pressed => button::Style {
                    background: Some(Background::Color(Color::new(0.85, 0.85, 0.85, 1.0))),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(4.0),
                    },
                    ..Default::default()
                },
            })
            .padding(Padding::new(4.0).left(24.0))
            .into(),
        Container::new(text(">"))
            .width(Length::Fixed(8.0))
            .padding(Padding::from([4.0, 8.0]))
            .into(),
    ])
    .into()
}
