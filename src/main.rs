mod camera;
mod icons;
mod plugin;
mod scene;
mod styles;
mod widget;

use iced::widget::{
    button,
    button::Style as ButtonStyle,
    center, column,
    container::{self, Style},
    mouse_area, opaque,
    pane_grid::{self, Axis, Configuration},
    row, stack, text,
    text_editor::{self, Content},
    text_input, Container, PaneGrid, Svg,
};
use iced::{
    border::Radius,
    keyboard::{self, key::Named, Key},
    Border, Length, Padding, Subscription, Task, Theme,
};
use iced::{Color, Element, Point, Settings};

use tokio::fs;

use std::{borrow::Cow, ffi::OsStr, path::PathBuf, sync::Arc};

use crate::{
    icons::IconStorage,
    plugin::{plugin_list, ExamplePlugin, Plugin, PluginHost, PluginId, PluginInfo},
    scene::{Rectangle, Scene, Spline},
    widget::canvas::canvas,
    widget::editor::NoteEditor,
};

pub enum PaneType {
    TextEditor,
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
    window: ActiveWindow,
    grid_state: pane_grid::State<PaneType>,
    icons: IconStorage,
    note_content: Content,
    opened_directory: Option<PathBuf>,
    current_file: Option<PathBuf>,
    directory_content: Option<Vec<PathBuf>>,
    plugin_host: PluginHost,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SendPluginAction {
        id: PluginId,
        action: Arc<plugin::Action>,
    },
    LoadPlugin(String, bool),
    SetActiveWindow(ActiveWindow),
    SetDebug(bool),
    SetDirectoryContent(Vec<PathBuf>),
    OpenedFile(Result<(PathBuf, String), ()>),
    PickFile(Option<PathBuf>),
    OpenFile(PathBuf),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action),
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
                .description("An example plugin that do nothing)"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        plugin_host.register_plugin(
            PluginInfo::new().name("ExamplePlugin2").id("core.example2"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        plugin_host.register_plugin(
            PluginInfo::new()
                .name("ExamplePlugin3")
                .id("core.example3")
                .description("Yet another example plugin that do nothing"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        let scene = Scene::new()
            .add_spline(Spline {
                points: vec![Point::new(50.0, 50.0), Point::new(60.0, 120.0)],
                color: Color::BLACK,
                width: 3.0,
            })
            .add_rectangle(Rectangle {
                position: Point::new(200.0, 150.0),
                w: 150.0,
                h: 200.0,
                color: Color::new(1.0, 0.0, 0.0, 1.0),
                width: 5.0,
            });

        Self {
            scene,
            window: ActiveWindow::None,
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
            debug: false,
        }
    }
}

async fn get_files(dir: impl Into<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let dir_path = dir.into();

    let mut dir_entries = fs::read_dir(dir_path).await.unwrap();

    while let Some(entry) = dir_entries.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    files
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
        let dir = app.opened_directory.clone();
        (
            app,
            if let Some(dir) = dir {
                Task::perform(get_files(dir), AppMessage::SetDirectoryContent)
            } else {
                Task::none()
            },
        )
    }

    fn title(&self) -> String {
        String::from("p3")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::SetActiveWindow(window) => {
                self.window = window;
            }

            AppMessage::SetDebug(state) => {
                self.debug = state;
            }

            AppMessage::LoadPlugin(id, load) => {
                if load {
                    self.plugin_host.load_plugin(&id);
                } else {
                    self.plugin_host.unload_plugin(&id);
                }
            }

            AppMessage::SendPluginAction { id: name, action } => {
                self.plugin_host.send_action(name, action);
            }

            AppMessage::TextEditorAction(action) => {
                self.note_content.perform(action);
            }

            AppMessage::SetDirectoryContent(content) => self.directory_content = Some(content),

            AppMessage::OpenedFile(result) => {
                if let Ok((path, content)) = result {
                    self.note_content = Content::with_text(&content);
                    self.current_file = Some(path);
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
                    return Task::perform(get_files(path), AppMessage::SetDirectoryContent);
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
                        ))
                        .padding(32.0)
                        .width(Length::Fixed(700.0)),
                    );
                    column![
                        Container::new(
                            text(if let Some(path) = &self.current_file {
                                String::from(
                                    path.file_name()
                                        .unwrap_or(OsStr::new(""))
                                        .to_str()
                                        .unwrap_or(""),
                                )
                            } else {
                                String::new()
                            })
                            .size(20.0)
                        )
                        .padding(8.0),
                        editor,
                    ]
                    .into()
                }

                PaneType::Canvas => {
                    let canvas_renderer =
                        Container::new(canvas(&self.scene).on_plugin_action(|a, b| {
                            AppMessage::SendPluginAction { id: a, action: b }
                        }));
                    canvas_renderer.into()
                }

                PaneType::FileExplorer => {
                    let mut files: Vec<Element<_>> = Vec::new();
                    if let Some(content) = &self.directory_content {
                        for file in content {
                            files.push(
                                button(text(file.file_name().unwrap().to_str().unwrap()))
                                    .on_press(AppMessage::OpenFile(file.clone()))
                                    .padding(0.0)
                                    .style(|theme: &Theme, _status| ButtonStyle {
                                        background: None,
                                        text_color: theme.palette().text,
                                        ..Default::default()
                                    })
                                    .into(),
                            );
                        }
                    }
                    column![
                        Container::new(
                            row![
                                button(Svg::new(self.icons.settings.clone()))
                                    .padding(Padding::new(2.0))
                                    .width(Length::Fixed(28.0))
                                    .height(Length::Fixed(28.0)),
                                button(Svg::new(self.icons.file_open.clone()))
                                    .on_press(AppMessage::PickFile(self.opened_directory.clone()))
                                    .padding(Padding::new(2.0))
                                    .width(Length::Fixed(28.0))
                                    .height(Length::Fixed(28.0)),
                                button(Svg::new(self.icons.plugins.clone()))
                                    .on_press(AppMessage::SetActiveWindow(ActiveWindow::Plugins))
                                    .padding(Padding::new(2.0))
                                    .width(Length::Fixed(28.0))
                                    .height(Length::Fixed(28.0)),
                                button(Svg::new(self.icons.action.clone()))
                                    .on_press(AppMessage::SetActiveWindow(ActiveWindow::Actions))
                                    .padding(Padding::new(2.0))
                                    .width(Length::Fixed(28.0))
                                    .height(Length::Fixed(28.0)),
                            ]
                            .spacing(4.0)
                            .padding(Padding::ZERO.bottom(4.0))
                        )
                        .style(|_| Color::new(0.95, 0.95, 0.95, 1.0).into()),
                        if files.is_empty() {
                            Container::new(text("Nothing"))
                        } else {
                            Container::new(column(files).spacing(8.0))
                        },
                    ]
                    .padding(4.0)
                    .into()
                }
            };
            Container::new(content)
                .style(|_theme| container::Style {
                    border: Border {
                        color: Color::new(0.85, 0.85, 0.85, 1.0),
                        width: 1.0,
                        radius: Radius::new(0.0),
                    },
                    ..Default::default()
                })
                .into()
        });
        let primary_screen = Container::new(grid)
            .width(Length::Fill)
            .height(Length::Fill);

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
                    row![text_input("Action", ""), button(text("Send"))].spacing(8.0),
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
            ActiveWindow::None => keyboard::on_key_press(|key, modifiers| match key.as_ref() {
                Key::Character("p") if modifiers.command() => {
                    Some(AppMessage::SetActiveWindow(ActiveWindow::Plugins))
                }

                Key::Character("r") if modifiers.command() => {
                    Some(AppMessage::SetActiveWindow(ActiveWindow::Actions))
                }

                Key::Character("o") if modifiers.command() => Some(AppMessage::PickFile(None)),

                _ => None,
            }),

            _ => keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
                Key::Named(Named::Escape) => Some(AppMessage::SetActiveWindow(ActiveWindow::None)),

                _ => None,
            }),
        }
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
        .theme(|_| Theme::CatppuccinLatte)
        .centered()
        .run_with(App::new)
}
