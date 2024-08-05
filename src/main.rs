#![windows_subsystem = "windows"]

mod icons;
mod notification;
mod plugin;
mod styles;
mod theming;
mod util;
mod widget;

use iced::{
    advanced::graphics::core::SmolStr,
    keyboard::{Key, Modifiers},
    Length, Padding, Subscription, Task,
};
use iced::{
    keyboard::on_key_press,
    widget::{
        column, container, horizontal_space, row, stack,
        text_editor::{self, Content},
        vertical_space, Container,
    },
};
use iced::{Element, Settings};

use widget::notificaton::notification_list;

use std::{borrow::Cow, collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};

use crate::{
    notification::{Notification, NotificationList},
    plugin::{ExamplePlugin, Hotkey, Plugin, PluginAction, PluginHost, PluginId, PluginInfo},
    theming::Theme,
    util::{
        delay, get_directory_content, get_file_name, load_theme_from_file, open_file, pick_file,
        save_file,
    },
    widget::pane::{file_explorer_pane, text_editor_pane},
};

pub type DocumentId = usize;

#[derive(Debug, Clone)]
pub enum PaneType {
    TextEditor(DocumentId),
}

pub struct DocumentHandler {
    pub text_content: Content,
    pub path: PathBuf,
    pub filename: Arc<String>,
    pub changed: bool,
}

pub struct App<'a> {
    theme: Theme<'a>,
    themes: HashMap<String, Theme<'a>>,
    documents: HashMap<DocumentId, DocumentHandler>,
    next_doc_id: DocumentId,
    opened_doc: DocumentId,
    opened_directory: Option<PathBuf>,
    directory_content: Option<Vec<PathBuf>>,
    plugin_host: PluginHost<AppMessage>,
    hotkeys: HashMap<Hotkey, AppMessage>,
    notifications: NotificationList,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SendPluginMessage {
        id: PluginId,
        message: Arc<plugin::PluginMessage>,
    },
    PluginAction(PluginId, PluginAction),
    LoadPlugin(PluginId, bool),
    SendNotification(Arc<Notification>),
    RemoveNotification(usize),
    ChangeTheme(String),
    LoadTheme(String, Theme<'static>),
    SetDirectoryContent(Vec<PathBuf>),
    OpenedFile(Result<(PathBuf, String), ()>),
    PickFile(Option<PathBuf>),
    FocusDocument(DocumentId),
    CloseDocument(DocumentId),
    OpenFile(PathBuf),
    SaveFile,
    SavedFile(DocumentId),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action, DocumentId),
    CosmicAction(cosmic_text::Action),
    OnKeyPress(Key, Modifiers),
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let mut plugin_host = PluginHost::new().on_plugin_action(AppMessage::PluginAction);
        plugin_host.register_plugin(
            PluginInfo::new()
                .name("ExamplePlugin")
                .id("core.example")
                .author("krozzzis")
                .version("1.0")
                .description("An example plugin that do nothing useful)"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        let mut hotkeys = HashMap::new();

        // Ctrl-o open file
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline("o")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::PickFile(None),
        );

        // Ctrl-s save file
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline("s")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::SaveFile,
        );

        // Ctrl-p set dark theme
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline("p")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::ChangeTheme("custom".to_owned()),
        );

        let mut themes = HashMap::new();
        themes.insert("light".to_owned(), Theme::default());
        themes.insert("dark".to_owned(), Theme::dark());

        Self {
            theme: Theme::default(),
            themes,
            documents: HashMap::new(),
            next_doc_id: 1,
            opened_doc: 0,
            plugin_host,
            directory_content: None,
            opened_directory: Some(PathBuf::from("./content/")),
            notifications: NotificationList::new(),
            hotkeys,
        }
    }
}

impl<'a> App<'a> {
    fn new() -> (Self, Task<AppMessage>) {
        let app = Self::default();
        let mut tasks = Vec::new();

        for id in app.plugin_host.get_plugin_ids() {
            let task = Task::done(AppMessage::LoadPlugin(id.clone(), true));
            tasks.push(task);
        }

        let dir = app.opened_directory.clone();
        if let Some(dir) = dir {
            tasks.push(Task::perform(
                get_directory_content(dir),
                AppMessage::SetDirectoryContent,
            ));
        }

        // Load theme from file
        tasks.push(Task::perform(
            load_theme_from_file("./themes/light.toml"),
            |theme| AppMessage::LoadTheme("custom".to_string(), theme.unwrap_or_default()),
        ));

        (app, Task::batch(tasks))
    }

    fn title(&self) -> String {
        String::from("p3")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::LoadTheme(name, theme) => {
                println!("Loaded theme {name}");
                self.themes.insert(name, theme);
            }

            AppMessage::SavedFile(id) => {
                if let Some(handler) = self.documents.get_mut(&id) {
                    handler.changed = false;
                }
            }

            AppMessage::SaveFile => {
                if let Some(handler) = self.documents.get(&self.opened_doc) {
                    let message = AppMessage::SavedFile(self.opened_doc);
                    return Task::perform(
                        save_file(handler.path.clone(), Arc::new(handler.text_content.text())),
                        move |_| message.clone(),
                    );
                }
            }
            AppMessage::FocusDocument(id) => {
                if id < self.next_doc_id {
                    self.opened_doc = id;

                    return Task::done(AppMessage::SendNotification(Arc::new(Notification {
                        text: format!("Focused document {id}",),
                        kind: notification::NotificationKind::None,
                    })));
                }
            }

            AppMessage::CloseDocument(id) => {
                if self.documents.contains_key(&id) {
                    self.documents.remove(&id);
                }
            }

            AppMessage::ChangeTheme(name) => {
                if let Some(theme) = self.themes.get(&name) {
                    self.theme = theme.clone();
                    return Task::done(AppMessage::SendNotification(Arc::new(Notification {
                        text: format!("Set theme: {}", name),
                        kind: notification::NotificationKind::None,
                    })));
                }
            }

            AppMessage::OnKeyPress(key, modifiers) => {
                if let Some(message) = self.on_key_press(key, modifiers) {
                    return Task::done(message);
                }
            }

            AppMessage::SendNotification(notificaton) => {
                let id = self.notifications.add(notificaton);
                return Task::perform(delay(5), move |_| AppMessage::RemoveNotification(id));
            }

            AppMessage::RemoveNotification(id) => {
                self.notifications.remove(id);
            }

            AppMessage::PluginAction(id, action) => match action {
                PluginAction::RegisterHotkey(hotkey, message) => {
                    self.hotkeys
                        .insert(hotkey, AppMessage::SendPluginMessage { id, message });
                }

                PluginAction::SendNotification(text) => {
                    return Task::done(AppMessage::SendNotification(Arc::new(Notification {
                        text: text.to_string(),
                        kind: notification::NotificationKind::None,
                    })))
                }
            },

            AppMessage::LoadPlugin(id, load) => {
                if load {
                    if let Some(message) = self.plugin_host.load_plugin(&id) {
                        return Task::done(message);
                    }
                } else if let Some(message) = self.plugin_host.unload_plugin(&id) {
                    return Task::done(message);
                }
            }

            AppMessage::SendPluginMessage {
                id: name,
                message: action,
            } => {
                if let Some(message) = self.plugin_host.send_message(name, action) {
                    return Task::done(message);
                }
            }

            AppMessage::TextEditorAction(action, document) => {
                if let Some(handler) = self.documents.get_mut(&document) {
                    if action.is_edit() {
                        handler.changed = true;
                    }
                    handler.text_content.perform(action);
                }
            }

            AppMessage::CosmicAction(_action) => {}

            AppMessage::SetDirectoryContent(content) => self.directory_content = Some(content),

            // TODO: Should accept an document id and fill it's handler with content
            AppMessage::OpenedFile(result) => {
                if let Ok((path, content)) = result {
                    let handler = DocumentHandler {
                        text_content: Content::with_text(&content),
                        path: path.clone(),
                        filename: Arc::new(get_file_name(&path)),
                        changed: false,
                    };

                    self.documents.insert(self.next_doc_id, handler);
                    let focus_doc = Task::done(AppMessage::FocusDocument(self.next_doc_id));
                    let notificaton =
                        Task::done(AppMessage::SendNotification(Arc::new(Notification {
                            text: format!(
                                "Opened file {} | ID: {}",
                                path.file_name()
                                    .unwrap_or(OsStr::new(""))
                                    .to_str()
                                    .unwrap_or(""),
                                self.next_doc_id,
                            ),
                            kind: notification::NotificationKind::None,
                        })));

                    self.next_doc_id += 1;

                    return Task::batch([focus_doc, notificaton]);
                }
            }

            AppMessage::PickFile(dir) => {
                return Task::perform(pick_file(dir), AppMessage::OpenedFile);
            }

            AppMessage::OpenFile(path) => {
                return Task::perform(open_file(path), AppMessage::OpenedFile);
            }

            AppMessage::OpenDirectory(path) => {
                if path.is_dir() {
                    self.opened_directory = Some(path.clone());
                    return Task::perform(
                        get_directory_content(path),
                        AppMessage::SetDirectoryContent,
                    );
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<AppMessage> {
        let editor = text_editor_pane(
            &self.documents,
            self.opened_doc,
            AppMessage::TextEditorAction,
            AppMessage::FocusDocument,
            AppMessage::CloseDocument,
            Some(AppMessage::PickFile(None)),
            &self.theme,
        );

        let file_explorer = file_explorer_pane(
            self.directory_content.as_ref(),
            self.documents
                .get(&self.opened_doc)
                .map(|handler| handler.path.clone()),
            AppMessage::OpenFile,
            &self.theme,
        );

        let grid = row![
            Container::new(file_explorer)
                .width(Length::Fixed(350.0))
                .padding(Padding::new(0.0).right(1.0))
                .style(|_| {
                    container::Style {
                        background: Some(self.theme.border_color.into()),
                        ..self.theme.container()
                    }
                }),
            Container::new(editor),
        ];

        let primary_screen = stack![
            Container::new(grid),
            row![
                horizontal_space(),
                column![
                    vertical_space(),
                    Container::new(notification_list(
                        &self.notifications.to_vec(),
                        Some(&self.theme)
                    ))
                    .padding(16.0)
                    .width(Length::Shrink)
                ],
            ],
        ];

        primary_screen.into()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        on_key_press(|key, modifiers| Some(AppMessage::OnKeyPress(key, modifiers)))
    }

    fn on_key_press(&mut self, key: Key, modifiers: Modifiers) -> Option<AppMessage> {
        for (hotkey, message) in &self.hotkeys {
            if hotkey.key == key && hotkey.modifiers == modifiers {
                return Some(message.clone());
            }
        }
        None
    }
}

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .font(Cow::Borrowed(styles::INTER_REGULAR_FONT_BYTES))
        .centered()
        .run_with(App::new)
}
