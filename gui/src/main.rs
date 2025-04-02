#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    keyboard::{on_key_press, Key},
    widget::{center, column},
    window::{close, drag, get_oldest, maximize, minimize},
    Element, Settings, Subscription, Task,
};

use log::info;
use std::path::PathBuf;
use theming::{stylesheet::StyleSheet, Theme};

use config::{AppConfig, GuiConfig, InterfaceMode};
use strelka_core::{smol_str::SmolStr, Modifiers};
use widget::{button::Button, container::background};

static DEFAULT_THEME: &str = "core.dark";
static THEME_PATH: &str = "./themes/dark/theme.kdl";
static APP_ICON: &[u8] = include_bytes!("../../contrib/icon.ico");

pub struct App {
    config: AppConfig,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    OnKeyPress(Key, iced::keyboard::Modifiers),
    WindowDrag,
    WindowClose,
    WindowMaximize,
    WindowMinimize,
    WindowCollapse,
    SetTheme(Theme),
    LoadTheme(PathBuf),
    None,
}

impl App {
    fn new() -> (Self, Task<AppMessage>) {
        let mut startup_tasks = Vec::new();

        let config = AppConfig {
            gui: GuiConfig {
                theme_id: SmolStr::new(DEFAULT_THEME),
                theme: Theme::default(),
                interface_mode: InterfaceMode::Simplified,
                scale_factor: 1.0,
            },
        };

        let app = Self {
            config,
            theme: Theme::default(),
        };

        let task = Task::done(AppMessage::LoadTheme(THEME_PATH.into()));
        startup_tasks.push(task);

        info!("App constructor done");
        (app, Task::batch(startup_tasks))
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        info!("Handling message: {message:?}");

        match message {
            AppMessage::None => {}

            AppMessage::SetTheme(theme) => {
                self.theme = theme;
            }

            AppMessage::LoadTheme(path) => {
                return Task::perform(StyleSheet::load(path), |stylesheet| {
                    if let Ok(stylesheet) = stylesheet {
                        AppMessage::SetTheme(Theme::from_stylesheet(stylesheet))
                    } else {
                        AppMessage::None
                    }
                });
            }

            AppMessage::OnKeyPress(key, modifiers) => {
                if let Some(message) = self.on_key_press(key, modifiers) {
                    return Task::done(message);
                }
            }

            AppMessage::WindowDrag => return get_oldest().then(|x| drag(x.unwrap())),
            AppMessage::WindowClose => return get_oldest().then(|x| close(x.unwrap())),
            AppMessage::WindowMaximize => return get_oldest().then(|x| maximize(x.unwrap(), true)),
            AppMessage::WindowMinimize => {
                return get_oldest().then(|x| maximize(x.unwrap(), false))
            }
            AppMessage::WindowCollapse => return get_oldest().then(|x| minimize(x.unwrap(), true)),
        }
        Task::none()
    }

    fn view(&self) -> Element<AppMessage, Theme> {
        background(center(column!(
            Button::new("Button1").on_press(AppMessage::None),
            iced::widget::Button::new("Button2").on_press(AppMessage::None),
        )))
        .into()
    }

    fn scale_factor(&self) -> f64 {
        self.config.gui.scale_factor
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        info!("Creating subscriptions");
        let key_press_listener =
            on_key_press(|key, modifiers| Some(AppMessage::OnKeyPress(key, modifiers)));

        Subscription::batch([key_press_listener])
    }

    fn on_key_press(
        &mut self,
        key: Key,
        modifiers: iced::keyboard::Modifiers,
    ) -> Option<AppMessage> {
        info!("Key press listener started");
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

            if *c == *SmolStr::new_static("r") {
                return Some(AppMessage::LoadTheme(THEME_PATH.into()));
            }
        }
        None
    }
}

fn main() -> iced::Result {
    env_logger::init();

    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .window(iced::window::Settings {
            icon: iced::window::icon::from_file_data(APP_ICON, None).ok(),
            decorations: false,
            ..Default::default()
        })
        .centered()
        .run_with(App::new)
}
