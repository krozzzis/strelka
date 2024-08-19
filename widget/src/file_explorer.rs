use std::{
    cmp::Reverse,
    collections::HashMap,
    path::{Path, PathBuf},
};

use iced::widget::container;
use iced::{widget::container::Style, Element, Length, Task};

use crate::list::{List, ListItem, TextButton};
use theming::{self, theme, Theme};

#[derive(Debug, Default)]
pub struct State {
    pub visible: bool,
    pub directory: PathBuf,
    pub selected: PathBuf,
    pub content: HashMap<PathBuf, Vec<PathBuf>>,
    pub list: List<Message>,
}

impl State {
    pub fn perform<Msg: Send + 'static>(
        &mut self,
        message: Message,
        to_msg: impl Fn(Message) -> Msg + Send + Sync + 'static,
    ) -> Task<Msg> {
        match message {
            Message::OpenFile(_) => Task::none(),

            Message::Toggle => {
                self.visible = !self.visible;
                Task::none()
            }

            Message::SetDirectory(path) => {
                self.directory = path;
                Task::none()
            }

            Message::SetSelected(path) => {
                self.selected = path;
                Task::none()
            }

            Message::GetFolderContent(dir) => {
                Task::perform(get_directory_content(dir.clone()), move |vector| {
                    to_msg(Message::AddFolderContent(dir.clone(), vector))
                })
            }

            Message::AddFolderContent(dir, mut vector) => {
                vector.sort_by_key(|a| Reverse(a.is_dir()));
                self.content.insert(dir, vector);

                self.list.elements.clear();
                for path in self.content.get(&self.directory).unwrap_or(&vec![]).iter() {
                    let button = TextButton::new(get_file_name(path).unwrap_or_default())
                        .on_click(Message::OpenFile((*path).clone()));
                    self.list.elements.push(ListItem::TextButton(button));
                }

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        container(self.list.view())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(theme!(file_explorer.padding))
            .style(|theme: &Theme| Style {
                text_color: Some(theme.file_explorer.text.into()),
                background: Some(theme.file_explorer.background.into()),
                ..Default::default()
            })
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GetFolderContent(PathBuf),
    AddFolderContent(PathBuf, Vec<PathBuf>),
    OpenFile(PathBuf),
    SetDirectory(PathBuf),
    SetSelected(PathBuf),
    Toggle,
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
