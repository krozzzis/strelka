#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use iced::{
    advanced::graphics::core::SmolStr,
    keyboard::{on_key_press, Key},
    widget::{
        column, horizontal_space, row, stack,
        text_editor::{self, Content},
        vertical_space, Container,
    },
    Element, Length, Settings, Subscription, Task,
};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::util::{delay, get_file_name, open_file, pick_file, save_file};

use plugin::{ExamplePlugin, Plugin, PluginAction, PluginHost, PluginId, PluginInfo};

use theming::{
    catalog::{get_themes, Catalog, ThemeID},
    metadata::ThemeMetadata,
    Theme,
};
use widget::{
    file_explorer,
    notificaton::notification_list,
    pane::{self, file_explorer::file_explorer_pane, pane_stack},
};

use core::{
    document::{DocumentHandler, DocumentId, DocumentStore},
    notification::{Notification, NotificationList},
    pane::PaneModel,
    HotKey, Modifiers,
};

#[derive(Debug, Clone)]
pub enum PaneType {
    TextEditor(DocumentId),
}

pub struct App {
    theme: Theme,
    theme_catalog: Catalog,
    default_theme: SmolStr,
    documents: DocumentStore<Content>,
    panes: PaneModel,
    opened_directory: Option<PathBuf>,
    plugin_host: PluginHost<AppMessage>,
    hotkeys: HashMap<HotKey, AppMessage>,
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
    LoadTheme(ThemeID),
    AddTheme(ThemeID, Box<Theme>, ThemeMetadata<'static>),
    OpenedFile(Result<(PathBuf, String), ()>),
    PickFile,
    CloseDocument(DocumentId),
    OpenFile(PathBuf),
    SaveFile(DocumentId),
    SavedFile(DocumentId),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action, DocumentId),
    FileExplorerAction(file_explorer::Message),
    OnKeyPress(Key, iced::keyboard::Modifiers),
    None,
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
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'o',
            },
            AppMessage::PickFile,
        );

        // Ctrl-d enable dark mode
        hotkeys.insert(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'd',
            },
            AppMessage::LoadTheme("core.dark".into()),
        );

        // Ctrl-p toggle file explorer
        hotkeys.insert(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'p',
            },
            AppMessage::FileExplorerAction(file_explorer::Message::Toggle),
        );

        let mut panes = PaneModel::new();
        {
            let id = panes.add(core::pane::Pane::NewDocument);
            panes.open(&id);
        }

        Self {
            theme: Theme::default(),
            theme_catalog: Catalog::new(),
            default_theme: SmolStr::from("core.light"),
            documents: DocumentStore::new(),
            panes,
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

        // Read themes from directory to stream
        let read_themes = Task::future(get_themes("./themes")).then(|stream| {
            // Add each theme from stream
            Task::run(stream, |(theme, metadata)| {
                AppMessage::AddTheme(metadata.id.to_string().into(), Box::new(theme), metadata)
            })
        });

        // Apply default theme
        let default_theme = app.default_theme.clone();
        let apply_default_theme =
            Task::perform(async move { default_theme }, AppMessage::LoadTheme);

        tasks.push(read_themes.chain(apply_default_theme));

        tasks.push(Task::done(AppMessage::FileExplorerAction(
            file_explorer::Message::GetFolderContent(
                app.opened_directory.clone().unwrap_or_default(),
            ),
        )));

        tasks.push(Task::done(AppMessage::OpenDirectory("./content".into())));

        (app, Task::batch(tasks))
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        println!("{message:?}");
        match message {
            AppMessage::None => {}

            AppMessage::FileExplorerAction(message) => match message {
                file_explorer::Message::OpenFile(path) => {
                    return Task::done(AppMessage::OpenFile(path))
                }

                _ => {
                    return self
                        .file_explorer
                        .perform(message, AppMessage::FileExplorerAction)
                }
            },

            AppMessage::AddTheme(id, theme, metadata) => {
                self.theme_catalog.insert(id, *theme, metadata);
            }

            AppMessage::LoadTheme(id) => {
                if let Some(theme) = self.theme_catalog.get_theme(id) {
                    self.theme = theme.clone();
                    {
                        if let Ok(mut global) = theming::THEME.try_write() {
                            *global = theme.clone();
                        }
                    }
                }
            }

            AppMessage::SavedFile(id) => {
                if let Some(handler) = self.documents.get_mut(&id) {
                    handler.changed = false;
                }
            }

            AppMessage::SaveFile(id) => {
                if let Some(handler) = self.documents.get(&id) {
                    let message = AppMessage::SavedFile(id);
                    return Task::perform(
                        save_file(handler.path.clone(), Arc::new(handler.text_content.text())),
                        move |_| message.clone(),
                    );
                }
            }

            AppMessage::CloseDocument(id) => {
                self.documents.remove(&id);
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

            AppMessage::PluginAction(_id, action) => match action {
                PluginAction::SendNotification(text) => {
                    return Task::done(AppMessage::SendNotification(Arc::new(
                        Notification::with_text(text.to_string()),
                    )))
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
                        filename: get_file_name(&path),
                        changed: false,
                    };

                    self.documents.add(handler);
                }
            }

            AppMessage::PickFile => {
                return Task::perform(pick_file(None), AppMessage::OpenedFile);
            }

            AppMessage::OpenFile(path) => {
                return Task::perform(open_file(path), AppMessage::OpenedFile);
            }

            AppMessage::OpenDirectory(path) => {
                if path.is_dir() {
                    self.opened_directory = Some(path.clone());

                    // Open directory in file explorer
                    return self.file_explorer.perform(
                        file_explorer::Message::SetDirectory(path),
                        AppMessage::FileExplorerAction,
                    );
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<AppMessage, Theme> {
        let file_explorer =
            file_explorer_pane(&self.file_explorer).map(AppMessage::FileExplorerAction);

        let mut grid_elements = Vec::new();
        if self.file_explorer.visible {
            grid_elements.push(
                Container::new(file_explorer)
                    .width(Length::Fixed(self.theme.file_explorer.width))
                    .into(),
            );
        }
        grid_elements.push(
            pane_stack::pane_stack(&self.panes).map(|msg| -> AppMessage {
                match msg {
                    pane_stack::Message::NewDocument(pane::new_document::Message::PickFile) => {
                        AppMessage::PickFile
                    }

                    _ => AppMessage::None,
                }
            }),
        );
        let grid = row(grid_elements);

        let primary_screen = stack![
            Container::new(grid),
            row![
                horizontal_space(),
                column![
                    vertical_space(),
                    Container::new(notification_list(&self.notifications.to_vec()))
                        .width(Length::Shrink)
                ],
            ],
        ];

        primary_screen.into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        on_key_press(|key, modifiers| Some(AppMessage::OnKeyPress(key, modifiers)))
    }

    fn on_key_press(
        &mut self,
        key: Key,
        modifiers: iced::keyboard::Modifiers,
    ) -> Option<AppMessage> {
        if let Key::Character(c) = key {
            let modifier = if modifiers.control() && modifiers.alt() {
                Modifiers::CtrlAlt
            } else if modifiers.control() {
                Modifiers::Ctrl
            } else if modifiers.alt() {
                Modifiers::Alt
            } else {
                Modifiers::None
            };

            let hotkey = HotKey {
                key: c.chars().next().unwrap_or_default(),
                modifiers: modifier,
            };

            if let Some(message) = self.hotkeys.get(&hotkey) {
                return Some(message.clone());
            }
        }
        None
    }
}

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .centered()
        .run_with(App::new)
}
