#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;

use command::ThemeLoadCommand;
use config::{load_config, ApplicationConfig};
use iced::{
    keyboard::{on_key_press, Key},
    widget::{center, column},
    window::{close, drag, get_oldest, maximize, minimize},
    Element, Length, Settings, Subscription, Task,
};

use log::info;
use std::sync::Arc;
use theming::Theme;

use strelka_core::{command::CommandRegistry, smol_str::SmolStr, Message, Modifiers, ThemeMessage};
use strelka_core::{CommandMessage, Theme as CoreTheme};
use widget::{
    button::{primary_button, Button},
    container::background,
};

static DEFAULT_THEME: &str = "core.dark";
static APP_ICON: &[u8] = include_bytes!("../../contrib/icon.ico");

pub struct App {
    config: ApplicationConfig,
    theme: CoreTheme,
    commands: Arc<CommandRegistry>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    OnKeyPress(Key, iced::keyboard::Modifiers),
    WindowDrag,
    WindowClose,
    WindowMaximize,
    WindowMinimize,
    WindowCollapse,
    CoreMessage(Message),
    None,
}

impl App {
    fn new(config: ApplicationConfig) -> (Self, Task<AppMessage>) {
        let mut startup_tasks = Vec::new();

        let commands = CommandRegistry::new();
        let _ = smol::block_on(commands.register("load_theme", ThemeLoadCommand::new()));

        let app = Self {
            config,
            commands: Arc::new(commands),
            theme: CoreTheme {
                inner: Arc::new(Theme::default()),
            },
        };

        let task = Task::done(AppMessage::CoreMessage(Message::Command(
            CommandMessage::CallCommand(SmolStr::new_static("load_theme"), Vec::new()),
        )));
        startup_tasks.push(task);

        info!("App constructor done");
        (app, Task::batch(startup_tasks))
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn handle_theme_message(&mut self, message: ThemeMessage) -> Task<AppMessage> {
        match message {
            ThemeMessage::SetTheme(theme) => {
                self.theme = theme.clone();
            }
        }
        Task::none()
    }

    fn handle_command_message(&mut self, message: CommandMessage) -> Task<AppMessage> {
        match message {
            CommandMessage::CallCommand(command, args) => {
                let commands = self.commands.clone();
                Task::perform(
                    async move {
                        let output = commands
                            .execute(&command, strelka_core::command::CommandArgs { args })
                            .await;
                        if let Ok(cmd) = output {
                            AppMessage::CoreMessage(cmd)
                        } else {
                            AppMessage::None
                        }
                    },
                    |msg| msg,
                )
            }
        }
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        info!("Handling message: {message:?}");

        match message {
            AppMessage::None => {}

            AppMessage::CoreMessage(message) => match message {
                Message::Command(command_message) => {
                    return self.handle_command_message(command_message)
                }
                Message::Theme(theme_message) => return self.handle_theme_message(theme_message),
                Message::None => {}
            },

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

    fn view(&self) -> Element<AppMessage, CoreTheme> {
        background(center(
            column!(
                primary_button("Button1")
                    .on_press(AppMessage::None)
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(60.0)),
                primary_button("Button2")
                    .on_press(AppMessage::None)
                    .width(Length::Fixed(80.0))
                    .height(Length::Fixed(40.0))
                    .padding(8.0),
            )
            .spacing(10.0),
        ))
        .into()
    }

    fn scale_factor(&self) -> f64 {
        self.config.scale_factor.into()
    }

    fn theme(&self) -> CoreTheme {
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
            let _modifier = if modifiers.control() && modifiers.alt() {
                Modifiers::CtrlAlt
            } else if modifiers.control() {
                Modifiers::Ctrl
            } else if modifiers.alt() {
                Modifiers::Alt
            } else {
                Modifiers::None
            };

            if *c == *SmolStr::new_static("r") {
                return Some(AppMessage::CoreMessage(Message::Command(
                    CommandMessage::CallCommand(
                        SmolStr::new_static("load_theme"),
                        vec![String::from("./themes/light/theme.kdl")],
                    ),
                )));
            }
        }
        None
    }
}

struct AppBoot {
    config: ApplicationConfig,
}

impl iced::application::Boot<App, AppMessage> for AppBoot {
    fn boot(&self) -> (App, Task<AppMessage>) {
        App::new(self.config.clone())
    }
}

fn main() -> iced::Result {
    env_logger::init();

    let config = load_config();

    let boot = AppBoot {
        config: config.clone(),
    };

    iced::application(boot, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .title(App::title)
        .scale_factor(App::scale_factor)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .window(iced::window::Settings {
            icon: iced::window::icon::from_file_data(APP_ICON, None).ok(),
            decorations: config.decorations,
            ..Default::default()
        })
        .centered()
        .run()
}
