use std::path::{Path, PathBuf};

use iced::{border::Radius, Border, Color, Element, Length, Shadow, Vector};
use iced::{
    widget::{component, container, stack, Component, Container, Space},
    Size,
};
use iced_aw::widgets::ContextMenu;

use crate::{
    theming::{self, Theme},
    widget::list::{list, ListItem},
};

pub struct FileExplorer<'a, Message> {
    pub files: Vec<&'a PathBuf>,
    pub dirs: Vec<&'a PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub on_click: Option<Box<dyn Fn(PathBuf) -> Message>>,
    pub theme: Option<&'a Theme<'a>>,
}

impl<'a, Message> FileExplorer<'a, Message> {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
            selected_file: None,
            on_click: None,
            theme: None,
        }
    }

    pub fn with_content(content: &'a [PathBuf]) -> Self {
        let files = content.iter().filter(|x| x.is_file()).collect();
        let dirs = content.iter().filter(|x| x.is_dir()).collect();
        Self {
            files,
            dirs,
            ..Self::new()
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

    pub fn select_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.selected_file = Some(path.into());
        self
    }

    pub fn select_file_maybe(mut self, path: Option<impl Into<PathBuf>>) -> Self {
        if let Some(path) = path {
            self.selected_file = Some(path.into());
        }
        self
    }

    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl<'a, Message> Default for FileExplorer<'a, Message> {
    fn default() -> Self {
        Self::new()
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

            Message::NewFile => {}
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let dirs = self.dirs.iter().map(|path| {
            ListItem::new(get_directory_name(path).unwrap_or(String::from("NaN")))
                .click(Message::OpenDir((*path).clone()))
                .theme(self.theme)
        });

        let files = self.files.iter().map(|path| {
            ListItem::new(get_file_name(path).unwrap_or(String::from("NaN")))
                .click(Message::OpenFile((*path).clone()))
                .selected(self.selected_file == Some((*path).clone()))
                .theme(self.theme)
        });

        let items = list(
            dirs.chain(files)
                .map(|x| x.into())
                .collect::<Vec<Element<_>>>(),
        );

        let fallback = &theming::FALLBACK;
        let generic = &self.theme.unwrap_or(fallback).theme.generic;

        let underlay = Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                text_color: Some(generic.text.into()),
                background: Some(generic.background2.into()),
                ..Default::default()
            });

        let menu = ContextMenu::new(underlay, move || {
            container(list(vec![ListItem::new("New file")
                .theme(self.theme)
                .click(Message::NewFile)
                .into()]))
            .padding(4.0)
            .width(Length::Fixed(200.0))
            .style(move |_| {
                let theme = self.theme.cloned().unwrap_or(Theme::default());
                container::Style {
                    background: Some(theme.surface.into()),
                    border: Border {
                        color: theme.border_color,
                        width: 1.0,
                        radius: Radius::new(theme.element_radius * 2.0),
                    },
                    shadow: Shadow {
                        color: Color::BLACK,
                        offset: Vector::new(2.0, 2.0),
                        blur_radius: 12.0,
                    },
                    ..Default::default()
                }
            })
            .into()
        });

        stack![menu, items].into()
    }

    fn size_hint(&self) -> iced::Size<Length> {
        Size::new(Length::Fill, Length::Fill)
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
    NewFile,
}

fn get_directory_name(path: &Path) -> Option<String> {
    path.parent()
        .and_then(|parent| parent.file_name())
        .and_then(|os_str| os_str.to_str())
        .map(String::from)
}

fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .map(String::from)
}
