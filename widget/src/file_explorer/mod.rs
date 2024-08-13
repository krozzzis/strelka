use std::{
    cmp::Reverse,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::{border::Radius, Border, Color, Element, Length, Shadow, Task, Vector};
use iced::{
    widget::{component, container, stack, Component, Container, Space},
    Size,
};
use iced_aw::widgets::ContextMenu;

use crate::list::{list, ListItem};
use theming::{self, Theme};

#[derive(Default, Debug)]
pub struct State {
    pub content: HashMap<PathBuf, Vec<PathBuf>>,
}

impl State {
    pub fn perform<Msg: Send + 'static>(
        &mut self,
        message: Message,
        to_msg: impl Fn(Message) -> Msg + Send + Sync + 'static,
    ) -> Task<Msg> {
        match message {
            Message::GetFolderContent(dir) => {
                Task::perform(get_directory_content(dir.clone()), move |vector| {
                    to_msg(Message::AddFolderContent(dir.clone(), vector))
                })
            }

            Message::AddFolderContent(dir, mut vector) => {
                vector.sort_by_key(|a| Reverse(a.is_dir()));
                self.content.insert(dir, vector);
                Task::none()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GetFolderContent(PathBuf),
    AddFolderContent(PathBuf, Vec<PathBuf>),
}

pub struct FileExplorer<'a, Message> {
    pub state: &'a State,
    pub path: Arc<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub on_click: Option<Box<dyn Fn(PathBuf) -> Message>>,
    pub theme: Option<&'a Theme>,
}

impl<'a, Message> FileExplorer<'a, Message> {
    pub fn new(dir: Arc<PathBuf>, state: &'a State) -> Self {
        Self {
            state,
            path: dir,
            selected_file: None,
            on_click: None,
            theme: None,
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

impl<'a, Msg> Component<Msg> for FileExplorer<'a, Msg> {
    type State = ();

    type Event = InternalMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Msg> {
        match event {
            InternalMessage::OpenDir(_path) => {}

            InternalMessage::OpenFile(path) => {
                if let Some(func) = &self.on_click {
                    return Some(func(path));
                }
            }

            InternalMessage::NewFile => {}
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let elements: Vec<Element<_>> = if let Some(content) = self.state.content.get(&*self.path) {
            content
                .iter()
                .map(|path| {
                    ListItem::new(get_file_name(path).unwrap_or(String::from("NaN")))
                        .click(InternalMessage::OpenFile((*path).clone()))
                        .selected(self.selected_file == Some((*path).clone()))
                        .theme(self.theme)
                })
                .map(|x| x.into())
                .collect()
        } else {
            vec![]
        };

        let theme = self.theme.unwrap_or(&theming::FALLBACK);

        let items = container(list(elements, theme)).padding(theme.file_explorer.padding);

        // let underlay = Container::new(Space::new(Length::Fill, Length::Fill))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .style(move |_: &iced::Theme| container::Style {
        //         text_color: Some(theme.file_explorer.text.into()),
        //         background: Some(theme.file_explorer.background.into()),
        //         ..Default::default()
        //     });
        //
        // let menu = ContextMenu::new(underlay, move || {
        //     container(list(
        //         vec![ListItem::new("New file")
        //             .theme(self.theme)
        //             .click(InternalMessage::NewFile)
        //             .into()],
        //         theme,
        //     ))
        //     .padding(theme.context_menu.padding + theme.context_menu.border_width)
        //     .width(Length::Fixed(theme.context_menu.width))
        //     .style(move |_: &iced::Theme| container::Style {
        //         background: Some(theme.context_menu.background.into()),
        //         border: Border {
        //             color: theme.context_menu.border_color.into(),
        //             width: theme.context_menu.border_width,
        //             radius: Radius::new(theme.context_menu.radius),
        //         },
        //         shadow: Shadow {
        //             color: Color::BLACK,
        //             offset: Vector::new(theme.context_menu.shadow_x, theme.context_menu.shadow_y),
        //             blur_radius: theme.context_menu.shadow_blur,
        //         },
        //         ..Default::default()
        //     })
        //     .into()
        // });

        stack![items].into()
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
pub enum InternalMessage {
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

async fn get_directory_content(dir: impl Into<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let dir_path = dir.into();

    let mut dir_entries = tokio::fs::read_dir(dir_path).await.unwrap();

    while let Some(entry) = dir_entries.next_entry().await.unwrap() {
        let path = entry.path();
        files.push(path);
    }

    files
}
