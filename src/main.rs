#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod notification;
mod plugin;
mod theming;
mod util;
mod widget;

use iced::{
    advanced::graphics::core::SmolStr,
    keyboard::{Key, Modifiers},
    widget::text_editor,
    Length, Subscription, Task,
};
use iced::{
    keyboard::on_key_press,
    widget::{
        column, horizontal_space, row, stack, text_editor::Content, vertical_space, Container,
    },
};
use iced::{Element, Settings};

use widget::notificaton::notification_list;

use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};

use crate::{
    notification::{Notification, NotificationList},
    plugin::{ExamplePlugin, Hotkey, Plugin, PluginAction, PluginHost, PluginId, PluginInfo},
    theming::{
        catalog::{Catalog, ThemeID},
        metadata::ThemeMetadata,
        Theme,
    },
    util::{delay, get_file_name, get_theme_metadatas, open_file, pick_file, save_file},
    widget::{
        file_explorer,
        pane::{file_explorer_pane, text_editor_pane},
    },
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

pub struct App {
    theme: Theme,
    theme_catalog: Catalog<'static>,
    documents: HashMap<DocumentId, DocumentHandler>,
    next_doc_id: DocumentId,
    opened_doc: DocumentId,
    opened_directory: Option<PathBuf>,
    plugin_host: PluginHost<AppMessage>,
    hotkeys: HashMap<Hotkey, AppMessage>,
    notifications: NotificationList,
    file_explorer: file_explorer::State,
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
    SetTheme(Box<Theme>),
    LoadTheme(ThemeID),
    AddTheme(ThemeID, ThemeMetadata<'static>),
    OpenedFile(Result<(PathBuf, String), ()>),
    PickFile(Option<PathBuf>),
    FocusDocument(DocumentId),
    CloseDocument(DocumentId),
    OpenFile(PathBuf),
    SaveFile,
    SavedFile(DocumentId),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action, DocumentId),
    FileExplorerAction(file_explorer::Message),
    OnKeyPress(Key, Modifiers),
}

impl Default for App {
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
            AppMessage::LoadTheme("light".into()),
        );

        Self {
            theme: Theme::default(),
            theme_catalog: Catalog::new(),
            documents: HashMap::new(),
            next_doc_id: 1,
            opened_doc: 0,
            plugin_host,
            opened_directory: Some(PathBuf::from("./content/")),
            notifications: NotificationList::new(),
            hotkeys,
            file_explorer: file_explorer::State::default(),
        }
    }
}

impl App {
    fn new() -> (Self, Task<AppMessage>) {
        let app = Self::default();
        let mut tasks = Vec::new();

        for id in app.plugin_host.get_plugin_ids() {
            let task = Task::done(AppMessage::LoadPlugin(id.clone(), true));
            tasks.push(task);
        }

        // Add theme from file
        tasks.push(
            Task::future(get_theme_metadatas("./themes")).then(|stream| {
                Task::run(stream, |meta: ThemeMetadata| {
                    AppMessage::AddTheme(meta.info.name.clone().into(), meta)
                })
            }),
        );

        tasks.push(Task::done(AppMessage::FileExplorerAction(
            file_explorer::Message::GetFolderContent(
                app.opened_directory.clone().unwrap_or_default(),
            ),
        )));

        (app, Task::batch(tasks))
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::FileExplorerAction(message) => {
                return self
                    .file_explorer
                    .perform(message, AppMessage::FileExplorerAction);
            }

            AppMessage::AddTheme(id, meta) => {
                self.theme_catalog.insert(id, meta);
            }

            AppMessage::LoadTheme(id) => {
                let path = self.theme_catalog.get_path(&id);
                if let Some(path) = path {
                    return Task::perform(Theme::from_file(path), move |theme| {
                        if let Ok(theme) = theme {
                            AppMessage::SetTheme(Box::new(theme))
                        } else {
                            AppMessage::SendNotification(Arc::new(Notification {
                                text: format!("Can't load theme {}", id),
                                kind: notification::NotificationKind::Error,
                            }))
                        }
                    });
                } else {
                    return Task::done(AppMessage::SendNotification(Arc::new(Notification {
                        text: format!("Can't load theme {}", id),
                        kind: notification::NotificationKind::Error,
                    })));
                }
            }

            AppMessage::SetTheme(theme) => {
                self.theme = *theme;
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
            Arc::new(self.opened_directory.clone().unwrap_or_default()),
            &self.file_explorer,
            self.documents
                .get(&self.opened_doc)
                .map(|handler| handler.path.clone()),
            AppMessage::OpenFile,
            &self.theme,
        );

        let grid = row![
            Container::new(file_explorer).width(Length::Fixed(self.theme.file_explorer.width)),
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
        .centered()
        .run_with(App::new)
}
