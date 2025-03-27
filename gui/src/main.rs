#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{AppConfig, GuiConfig, InterfaceMode};
use iced::{
    keyboard::{on_key_press, Key},
    widget::text,
    window::{close, drag, get_oldest, maximize, minimize},
    Element, Settings, Subscription, Task,
};

use log::info;
use theming::Theme;

use core::{smol_str::SmolStr, Modifiers};

static DEFAULT_THEME: &str = "core.dark";
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
    None,
}

impl App {
    fn new() -> (Self, Task<AppMessage>) {
        let startup_tasks = Vec::new();

        let config = AppConfig {
            gui: GuiConfig {
                theme_id: SmolStr::new(DEFAULT_THEME),
                theme: Theme::default(),
                interface_mode: InterfaceMode::Simplified,
                scale_factor: 0.8,
            },
        };

        let app = Self {
            config,
            theme: theming::FALLBACK,
        };

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
        text("Hello").into()
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
