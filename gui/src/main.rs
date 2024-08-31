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

use plugin::{ExamplePlugin, Plugin, PluginHost, PluginId, PluginInfo};

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
    action::{Action, DocumentAction, FileAction, GenericAction, PaneAction},
    document::{DocumentHandler, DocumentId, DocumentStore},
    notification::{Notification, NotificationList},
    pane::{Pane, PaneModel},
    HotKey, Modifiers,
};

type State<'a> = core::State<'a, Content>;
type HotKeyHandler = dyn Fn(State) -> AppMessage;

pub struct App {
    theme: Theme,
    theme_catalog: Catalog,
    default_theme: SmolStr,
    documents: DocumentStore<Content>,
    panes: PaneModel,
    opened_directory: PathBuf,
    plugin_host: PluginHost,
    hotkeys: HashMap<HotKey, Box<HotKeyHandler>>,
    notifications: NotificationList,
    file_explorer: file_explorer::State,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    LoadPlugin(PluginId, bool),
    SendNotification(Arc<Notification>),
    RemoveNotification(usize),
    LoadTheme(ThemeID),
    AddTheme(ThemeID, Box<Theme>, ThemeMetadata<'static>),
    OpenedFile(Result<(PathBuf, String), ()>),
    GenericAction(GenericAction),
    Action(Action),
    SavedFile(DocumentId),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action, DocumentId),
    FileExplorerAction(file_explorer::Message),
    OnKeyPress(Key, iced::keyboard::Modifiers),
    None,
}

impl Default for App {
    fn default() -> Self {
        let mut plugin_host = PluginHost::new();
        plugin_host.register_plugin(
            PluginInfo::new()
                .name("ExamplePlugin")
                .id("core.example")
                .author("krozzzis")
                .version("1.0")
                .description("An example plugin that do nothing useful)"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        let mut panes = PaneModel::new();
        {
            let id = panes.add(Pane::NewDocument);
            panes.open(&id);
        }

        let mut app = Self {
            theme: Theme::default(),
            theme_catalog: Catalog::new(),
            default_theme: SmolStr::from("core.light"),
            documents: DocumentStore::new(),
            panes,
            plugin_host,
            opened_directory: PathBuf::from("./content")
                .canonicalize()
                .unwrap_or_default(),
            notifications: NotificationList::new(),
            hotkeys: HashMap::new(),
            file_explorer: file_explorer::State::default(),
        };

        // Ctrl-o open file
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'o',
            },
            |_: State| AppMessage::Action(Action::new(FileAction::PickFile)),
        );

        // Ctrl-d enable dark mode
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'd',
            },
            |_: State| AppMessage::LoadTheme("core.dark".into()),
        );

        // Ctrl-p toggle file explorer
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'p',
            },
            |_: State| AppMessage::FileExplorerAction(file_explorer::Message::Toggle),
        );

        // Ctrl-t open new document tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 't',
            },
            |_: State| AppMessage::Action(Action::new(PaneAction::Add(Pane::NewDocument))),
        );

        // Ctrl-w close open tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'w',
            },
            |state: State| {
                if let Some(id) = state.panes.get_open_id() {
                    AppMessage::Action(Action::new(PaneAction::Close(*id)))
                } else {
                    AppMessage::None
                }
            },
        );

        // Ctrl-b open experimental buffer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'b',
            },
            |_state: State| AppMessage::Action(Action::new(PaneAction::Add(Pane::Buffer))),
        );

        app
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

        if let Some(home) = dirs::home_dir() {
            let mut workdir = home;
            workdir.push("strelka");

            let file_explorer_content = Task::done(AppMessage::FileExplorerAction(
                file_explorer::Message::GetFolderContent(workdir.clone()),
            ));

            tasks.push(Task::perform(util::init_workdir(workdir.clone()), |_| {
                AppMessage::None
            }));
            tasks.push(Task::done(AppMessage::OpenDirectory(workdir)).chain(file_explorer_content));
        }

        tasks.push(Task::done(AppMessage::FileExplorerAction(
            file_explorer::Message::GetFolderContent(app.opened_directory.clone()),
        )));

        (app, Task::batch(tasks))
    }

    fn get_state(&self) -> State {
        State {
            documents: &self.documents,
            panes: &self.panes,
            working_directory: self.opened_directory.clone(),
        }
    }

    fn add_hotkey<F>(&mut self, hotkey: HotKey, func: F)
    where
        F: Fn(State) -> AppMessage + 'static,
    {
        self.hotkeys.insert(hotkey, Box::new(func));
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn perform_action(&mut self, action: GenericAction) -> Task<AppMessage> {
        match action {
            GenericAction::File(action) => match action {
                FileAction::PickFile => {
                    return Task::perform(pick_file(None), AppMessage::OpenedFile)
                }
                FileAction::OpenFileCurrentTab(path) => {
                    return Task::perform(open_file(path), AppMessage::OpenedFile)
                }
                FileAction::OpenFileForceCurrentTab(path) => {
                    return Task::perform(open_file(path), AppMessage::OpenedFile)
                }
                FileAction::OpenFileNewTab(path) => {
                    return Task::perform(open_file(path), AppMessage::OpenedFile)
                }
            },
            GenericAction::Pane(action) => match action {
                PaneAction::Close(id) => {
                    let pane = self.panes.remove(&id);

                    // Close document if Editor pane was closed
                    if let Some(Pane::Editor(doc_id)) = pane {
                        self.documents.remove(&doc_id);
                    }

                    // If there no panes left, create a NewDocument one
                    if self.panes.count() == 0 {
                        let id = self.panes.add(Pane::NewDocument);
                        self.panes.open(&id);
                    }
                }
                PaneAction::Open(id) => self.panes.open(&id),
                PaneAction::Add(pane) => {
                    let id = self.panes.add(pane);
                    self.panes.open(&id);
                }
                PaneAction::Replace(id, pane) => {
                    self.panes.replace(&id, pane);
                }
            },
            GenericAction::Document(action) => match action {
                DocumentAction::Add(handler) => {
                    let content = Content::with_text(&handler.text_content);
                    let handler = DocumentHandler {
                        text_content: content,
                        path: handler.path.clone(),
                        filename: handler.filename.clone(),
                        changed: handler.changed,
                    };
                    self.documents.add(handler);
                }
                DocumentAction::Open(id) => {
                    let pane = Pane::Editor(id);
                    return Task::done(AppMessage::Action(Action::new(PaneAction::Add(pane))));
                }
                DocumentAction::Save(id) => {
                    if let Some(handler) = self.documents.get(&id) {
                        let message = AppMessage::SavedFile(id);
                        return Task::perform(
                            save_file(handler.path.clone(), Arc::new(handler.text_content.text())),
                            move |_| message.clone(),
                        );
                    }
                }
                DocumentAction::Remove(id) => {
                    self.documents.remove(&id);
                }
            },
        }
        Task::none()
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        #[cfg(debug_assertions)]
        println!("Message: {message:?}");
        match message {
            AppMessage::None => {}

            AppMessage::Action(action) => {
                let action = self.plugin_host.process_action(action);
                let mut tasks = Vec::new();
                for generic in action.iter() {
                    tasks.push(self.perform_action(generic.clone()));
                }

                return Task::batch(tasks);
            }

            AppMessage::GenericAction(action) => return self.perform_action(action),

            AppMessage::FileExplorerAction(message) => match message {
                file_explorer::Message::OpenFile(path) => {
                    return Task::done(AppMessage::Action(Action::new(
                        FileAction::OpenFileCurrentTab(path),
                    )))
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

            AppMessage::LoadPlugin(id, load) => {
                if load {
                    self.plugin_host.load_plugin(&id);
                } else {
                    self.plugin_host.unload_plugin(&id);
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

                    let doc_id = self.documents.add(handler);
                    let pane = Pane::Editor(doc_id);

                    // If opened pane is NewDocument, replace it with Editor pane
                    // otherwise add new one with Editor
                    if let Some(&Pane::NewDocument) = self.panes.get_open() {
                        self.panes
                            .replace(&self.panes.get_open_id().cloned().unwrap_or(0usize), pane);
                    } else {
                        let pane_id = self.panes.add(pane);
                        self.panes.open(&pane_id);
                    }
                }
            }

            AppMessage::OpenDirectory(path) => {
                if path.is_dir() {
                    let path: PathBuf = path.canonicalize().unwrap_or_default();

                    self.opened_directory.clone_from(&path);

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
        let file_explorer = file_explorer_pane(
            State {
                documents: &self.documents,
                panes: &self.panes,
                working_directory: self.opened_directory.clone(),
            },
            &self.file_explorer,
        )
        .map(AppMessage::FileExplorerAction);

        let mut grid_elements = Vec::new();
        if self.file_explorer.visible {
            grid_elements.push(
                Container::new(file_explorer)
                    .width(Length::Fixed(self.theme.file_explorer.width))
                    .into(),
            );
        }
        grid_elements.push(
            pane_stack::pane_stack(State {
                documents: &self.documents,
                panes: &self.panes,
                working_directory: self.opened_directory.clone(),
            })
            .map(|msg| -> AppMessage {
                match msg {
                    pane_stack::Message::NewDocument(pane::new_document::Message::PickFile) => {
                        AppMessage::Action(Action::new(FileAction::PickFile))
                    }

                    pane_stack::Message::NewPane(pane) => {
                        AppMessage::Action(Action::new(PaneAction::Add(pane)))
                    }

                    pane_stack::Message::OpenPane(id) => {
                        AppMessage::Action(Action::new(PaneAction::Open(id)))
                    }

                    pane_stack::Message::ClosePane(id) => {
                        AppMessage::Action(Action::new(PaneAction::Close(id)))
                    }

                    pane_stack::Message::TextEditor(
                        id,
                        pane::text_editor::Message::EditorAction(action),
                    ) => AppMessage::TextEditorAction(action, id),
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

            if let Some(func) = self.hotkeys.get(&hotkey) {
                return Some(func(self.get_state()));
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
