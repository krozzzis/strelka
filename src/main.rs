#![windows_subsystem = "windows"]

mod camera;
mod icons;
mod notification;
mod plugin;
mod scene;
mod styles;
mod theme;
mod widget;

use iced::{
    advanced::graphics::core::SmolStr,
    border::Radius,
    keyboard::{self, key::Named, Key},
    widget::button,
    Background, Border, Length, Padding, Subscription, Task,
};
use iced::{
    keyboard::Modifiers,
    widget::{
        center, column,
        container::{self, Style},
        horizontal_space, mouse_area, opaque,
        pane_grid::{self, Axis, Configuration},
        row, stack, text,
        text_editor::{self, Content},
        text_input, vertical_space, Button, Container, PaneGrid, Svg,
    },
};
use iced::{Color, Element, Settings};

use tokio::fs;
use widget::notificaton::notification_list;

use std::{
    borrow::Cow, collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc, time::Duration,
};

use crate::{
    icons::IconStorage,
    notification::{Notification, NotificationList},
    plugin::{
        plugin_list, ExamplePlugin, Hotkey, Plugin, PluginAction, PluginHost, PluginId, PluginInfo,
    },
    scene::{Rectangle, Scene},
    theme::Theme,
    widget::{
        canvas::canvas, cosmic::cosmic_editor, editor::NoteEditor, file_explorer::FileExplorer,
    },
};

pub enum PaneType {
    TextEditor,
    Cosmic,
    Canvas,
    FileExplorer,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveWindow {
    None,
    Plugins,
    Actions,
}

pub struct App {
    debug: bool,
    scene: Scene,
    theme: Theme,
    themes: HashMap<String, Theme>,
    window: ActiveWindow,
    grid_state: pane_grid::State<PaneType>,
    icons: IconStorage,
    note_content: Content,
    opened_directory: Option<PathBuf>,
    current_file: Option<PathBuf>,
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
    SetActiveWindow(ActiveWindow),
    SetDebug(bool),
    ChangeTheme(String),
    SetDirectoryContent(Vec<PathBuf>),
    OpenedFile(Result<(PathBuf, String), ()>),
    PickFile(Option<PathBuf>),
    OpenFile(PathBuf),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action),
    CosmicAction(cosmic_text::Action),
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

        let scene = Scene::new();

        let mut hotkeys = HashMap::new();

        // Ctrl-o open file
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline("o")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::PickFile(None),
        );

        // Ctrl-, plugins list
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline(",")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::SetActiveWindow(ActiveWindow::Plugins),
        );

        // Ctrl-p set dark theme
        hotkeys.insert(
            Hotkey {
                key: Key::Character(SmolStr::new_inline("p")),
                modifiers: Modifiers::CTRL,
            },
            AppMessage::ChangeTheme("dark".to_owned()),
        );

        let mut themes = HashMap::new();
        themes.insert("light".to_owned(), Theme::default());
        themes.insert("dark".to_owned(), Theme::dark());

        Self {
            scene,
            window: ActiveWindow::None,
            theme: Theme::default(),
            themes,
            plugin_host,
            grid_state: pane_grid::State::with_configuration(Configuration::Split {
                axis: Axis::Vertical,
                ratio: 0.25,
                a: Box::new(Configuration::Pane(PaneType::FileExplorer)),
                b: Box::new(Configuration::Pane(PaneType::TextEditor)),
            }),
            icons: IconStorage::new(),
            note_content: Content::with_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 

Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 

Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
            directory_content: None,
            current_file: None,
            opened_directory: Some(PathBuf::from("./content/")),
            notifications: NotificationList::new(),
            hotkeys,
            debug: false,
        }
    }
}

async fn get_directory_content(dir: impl Into<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let dir_path = dir.into();

    let mut dir_entries = fs::read_dir(dir_path).await.unwrap();

    while let Some(entry) = dir_entries.next_entry().await.unwrap() {
        let path = entry.path();
        files.push(path);
    }

    files
}

async fn delay(secs: u64) {
    tokio::time::sleep(Duration::new(secs, 0)).await
}

async fn open_file(path: impl Into<PathBuf>) -> Result<(PathBuf, String), ()> {
    let path = path.into();
    let content = fs::read_to_string(&path).await.map_err(|_| ())?;
    Ok((path, content))
}

async fn pick_file(directory: Option<PathBuf>) -> Result<(PathBuf, String), ()> {
    let handler = if let Some(dir) = directory {
        rfd::AsyncFileDialog::new().set_directory(dir)
    } else {
        rfd::AsyncFileDialog::new()
    }
    .pick_file()
    .await;

    if let Some(path) = handler {
        let content = open_file(path.path()).await.map_err(|_| ())?;
        Ok(content)
    } else {
        Err(())
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

        let dir = app.opened_directory.clone();
        if let Some(dir) = dir {
            tasks.push(Task::perform(
                get_directory_content(dir),
                AppMessage::SetDirectoryContent,
            ));
        }

        (app, Task::batch(tasks))
    }

    fn title(&self) -> String {
        String::from("p3")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
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

            AppMessage::SetActiveWindow(window) => {
                self.window = window;
            }

            AppMessage::SetDebug(state) => {
                self.debug = state;
            }

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

            AppMessage::TextEditorAction(action) => {
                self.note_content.perform(action);
            }

            AppMessage::CosmicAction(_action) => {}

            AppMessage::SetDirectoryContent(content) => self.directory_content = Some(content),

            AppMessage::OpenedFile(result) => {
                if let Ok((path, content)) = result {
                    self.note_content = Content::with_text(&content);
                    self.current_file = Some(path.clone());
                    return Task::done(AppMessage::SendNotification(Arc::new(Notification {
                        text: format!(
                            "Opened file {}",
                            path.file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_str()
                                .unwrap_or("")
                        ),
                        kind: notification::NotificationKind::None,
                    })));
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
        let grid = PaneGrid::new(&self.grid_state, |_id, pane, _is_maximized| {
            let content: Element<_> = match *pane {
                PaneType::TextEditor => {
                    let editor = center(
                        Container::new(NoteEditor::new(
                            &self.note_content,
                            AppMessage::TextEditorAction,
                        ).theme(&self.theme))
                        .padding(Padding::from([0.0, 32.0]))
                        .width(Length::Fixed(700.0)),
                    );
                    Container::new(column![
                        Container::new(
                            text(if let Some(path) = &self.current_file {
                                path.file_name()
                                    .unwrap_or(OsStr::new(""))
                                    .to_str()
                                    .unwrap_or("")
                                    .to_owned()
                            } else {
                                String::new()
                            })
                            .size(20.0)
                        )
                        .padding(8.0)
                        .style(move |_| {
                            container::Style {
                                background: Some(self.theme.background.into()),
                                text_color: Some(self.theme.text),
                                ..Default::default()
                            }
                        }),
                        editor,
                    ])
                    .style(move |_| container::Style {
                        background: Some(self.theme.background.into()),
                        text_color: Some(self.theme.text),
                        ..Default::default()
                    })
                    .into()
                }

                PaneType::Canvas => {
                    let canvas_renderer =
                        Container::new(canvas(&self.scene).on_plugin_action(|a, b| {
                            AppMessage::SendPluginMessage { id: a, message: b }
                        }));
                    canvas_renderer.into()
                }

                PaneType::Cosmic => {
                    let canvas_renderer = Container::new(cosmic_editor().width(Length::Fill));
                    canvas_renderer.into()
                }

                PaneType::FileExplorer => Container::new(column![
                    Container::new(
                        FileExplorer::with_content_maybe(self.directory_content.as_deref())
                            .opened_file_maybe(self.current_file.as_deref())
                            .file_click(AppMessage::OpenFile)
                            .theme(&self.theme)
                    )
                    .height(Length::Fill),
                    Container::new(
                        Button::new(Svg::new(self.icons.settings.clone()))
                            .padding(Padding::new(2.0))
                            .width(Length::Fixed(28.0))
                            .height(Length::Fixed(28.0))
                            .on_press(AppMessage::SetActiveWindow(ActiveWindow::Plugins))
                            .style(move |_, status| {
                                match status {
                                    button::Status::Active | button::Status::Disabled => {
                                        button::Style {
                                            background: None,
                                            ..Default::default()
                                        }
                                    }

                                    button::Status::Hovered | button::Status::Pressed => {
                                        button::Style {
                                            background: Some(self.theme.selected.into()),
                                            border: Border {
                                                color: Color::TRANSPARENT,
                                                width: 0.0,
                                                radius: Radius::new(4.0),
                                            },
                                            ..Default::default()
                                        }
                                    }
                                }
                            })
                    )
                    .width(Length::Fill)
                    .padding(8.0)
                    .style(move |_| {
                        container::Style {
                            background: Some(self.theme.background2.into()),
                            ..Default::default()
                        }
                    }),
                ])
                .style(move |_| container::Style {
                    background: Some(self.theme.background.into()),
                    text_color: Some(self.theme.text),
                    ..Default::default()
                })
                .into(),
            };
            Container::new(content)
                .style(move |_| container::Style {
                    border: Border {
                        color: self.theme.border_color,
                        width: 1.0,
                        radius: Radius::new(0.0),
                    },
                    ..Default::default()
                })
                .into()
        });

        let primary_screen = stack![
            Container::new(grid)
                .width(Length::Fill)
                .height(Length::Fill),
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

        match self.window {
            ActiveWindow::Plugins => {
                let plugin_list = Container::new(plugin_list(
                    self.plugin_host.get_plugin_entries(),
                    AppMessage::LoadPlugin,
                ))
                .style(|_theme| {
                    let mut style: Style = Color::WHITE.into();
                    style.border.radius = Radius::new(16.0);
                    style
                })
                .width(Length::Fixed(512.0))
                .padding(16.0);

                modal(
                    primary_screen,
                    plugin_list,
                    AppMessage::SetActiveWindow(ActiveWindow::None),
                )
            }

            ActiveWindow::Actions => {
                let action_entry = Container::new(
                    row![text_input("Action", ""), Button::new(text("Send"))].spacing(8.0),
                )
                .style(|_theme| {
                    let mut style: Style = Color::WHITE.into();
                    style.border.radius = Radius::new(16.0);
                    style
                })
                .width(Length::Fixed(512.0))
                .padding(16.0);

                modal(
                    primary_screen,
                    action_entry,
                    AppMessage::SetActiveWindow(ActiveWindow::None),
                )
            }
            _ => primary_screen.into(),
        }
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        match self.window {
            ActiveWindow::None => keyboard::on_key_press(|key, modifiers| {
                Some(AppMessage::OnKeyPress(key, modifiers))
            }),

            _ => keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
                Key::Named(Named::Escape) => Some(AppMessage::SetActiveWindow(ActiveWindow::None)),

                _ => None,
            }),
        }
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

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        mouse_area(center(opaque(content)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }))
        .on_press(on_blur)
    ]
    .into()
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
