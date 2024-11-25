#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{AppConfig, GuiConfig, InterfaceMode};
use iced::{
    keyboard::{on_key_press, Key},
    window::{close, drag, get_oldest, maximize, minimize, Id},
    Element, Settings, Subscription, Task,
};
use log::{debug, info};
use state::{
    actors::{DocumentActor, FileActor, PaneActor, PluginHostActor, ThemeActor},
    ActionBrocker,
};
use tokio::sync;
use tokio::sync::mpsc::{self, channel};

use std::collections::HashMap;

use plugin::{ExamplePlugin, Plugin, PluginHost, PluginInfo};

use theming::Theme;
use widget::pane::pane_stack::{simplified_pane_stack, Msg};

use action::{Action, ActionTransport, FileAction, IntoAction, PaneAction, ThemeAction};
use core::{pane::Pane, smol_str::SmolStr, HotKey, Modifiers};

static DEFAULT_THEME: &str = "core.dark";
static APP_ICON: &[u8] = include_bytes!("../../contrib/icon.ico");

pub type HotKeyHandler = Box<dyn Fn() -> Action>;

pub struct App {
    config: AppConfig,
    brocker_tx: sync::mpsc::Sender<ActionTransport>,
    hotkeys: HashMap<HotKey, HotKeyHandler>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Action(Action),
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
        let mut startup_tasks = Vec::new();

        // Actor channels
        let (document_tx, document_rx) = channel(10);
        let (file_tx, file_rx) = channel(10);
        let (pane_tx, pane_rx) = channel(10);
        let (theme_tx, theme_rx) = channel(10);
        let (plugins_tx, plugins_rx) = channel(10);
        let (brocker_tx, brocker_rx) = channel(10);

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

        plugin_host.set_brocker(brocker_tx.clone());

        // Actors
        let mut document_actor = DocumentActor::new(document_rx, brocker_tx.clone());
        let mut pane_actor = PaneActor::new(pane_rx, brocker_tx.clone());
        let mut file_actor = FileActor::new(file_rx, brocker_tx.clone());
        let mut theme_actor = ThemeActor::new(theme_rx, brocker_tx.clone());
        let mut plugin_host_actor =
            PluginHostActor::new(plugins_rx, brocker_tx.clone()).set_host(plugin_host);

        let mut brocker = ActionBrocker::new(brocker_rx)
            .document_actor(document_tx.clone())
            .file_actor(file_tx.clone())
            .pane_actor(pane_tx.clone())
            .theme_actor(theme_tx.clone())
            .plugin_host_actor(plugins_tx.clone());

        tokio::spawn(async move { brocker.run().await });

        tokio::spawn(async move { document_actor.run().await });
        tokio::spawn(async move { pane_actor.run().await });
        tokio::spawn(async move { file_actor.run().await });
        tokio::spawn(async move { theme_actor.run().await });
        tokio::spawn(async move { plugin_host_actor.run().await });

        let config = AppConfig {
            gui: GuiConfig {
                theme_id: SmolStr::new(DEFAULT_THEME),
                theme: Theme::default(),
                interface_mode: InterfaceMode::Simplified,
                scale_factor: 1.0,
            },
        };

        let mut app = Self {
            config,
            brocker_tx,
            hotkeys: HashMap::new(),
        };

        // Ctrl+o open file
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'o',
            },
            || FileAction::PickFile.into_action(),
        );

        // Ctrl+t open new document tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 't',
            },
            || PaneAction::Add(Pane::NewDocument).into_action(),
        );

        // Ctrl+, open config viewer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: ',',
            },
            || PaneAction::Add(Pane::Config).into_action(),
        );

        // Ctrl+Alt+m make theme index
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::CtrlAlt,
                key: 'm',
            },
            || ThemeAction::MakeIndex.into_action(),
        );

        // Ctrl+Alt+l set light theme
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::CtrlAlt,
                key: 'l',
            },
            || ThemeAction::SetTheme(SmolStr::new("core.light")).into_action(),
        );

        {
            let make_index = Task::done(AppMessage::Action(ThemeAction::MakeIndex.into_action()));
            let set_theme = Task::done(AppMessage::Action(
                ThemeAction::SetTheme(SmolStr::new(DEFAULT_THEME)).into_action(),
            ));
            let chain = make_index.chain(set_theme);
            startup_tasks.push(chain);
        }

        {
            let new_doc = Task::done(AppMessage::Action(
                PaneAction::Add(Pane::NewDocument).into_action(),
            ));
            startup_tasks.push(new_doc);
        }

        info!("App constructor done");
        (app, Task::batch(startup_tasks))
    }

    fn add_hotkey<T: 'static + Fn() -> Action>(&mut self, hotkey: HotKey, handler: T) {
        info!("Added hotkey {hotkey:?}");
        self.hotkeys.insert(hotkey, Box::new(handler));
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn perform_action(&mut self, action: Action) -> Task<AppMessage> {
        let brocker_tx = self.brocker_tx.clone();
        Task::perform(
            async move {
                let _ = brocker_tx.send(action.into_transport()).await;
            },
            |_| AppMessage::None,
        )
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        info!("Handling message: {message:?}");

        match message {
            AppMessage::None => {}

            AppMessage::Action(action) => return self.perform_action(action),

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
        simplified_pane_stack(self.brocker_tx.clone()).map(|x| match x {
            Msg::None => AppMessage::None,
            Msg::Action(action) => AppMessage::Action(action),
            Msg::Close => AppMessage::WindowClose,
            Msg::Maximize => AppMessage::WindowMaximize,
            Msg::Collapse => AppMessage::WindowCollapse,
            Msg::Drag => AppMessage::WindowDrag,
        })
    }

    fn scale_factor(&self) -> f64 {
        self.config.gui.scale_factor
    }

    fn theme(&self) -> Theme {
        let (tx, mut rx) = mpsc::channel(1);
        let get_theme = ThemeAction::GetCurrentTheme(tx).into_transport();
        let _ = self.brocker_tx.blocking_send(get_theme);
        if let Some(theme) = rx.blocking_recv() {
            theme
        } else {
            self.config.gui.theme.clone()
        }
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

            let hotkey = HotKey {
                key: c.chars().next().unwrap_or_default(),
                modifiers: modifier,
            };

            debug!("Pressed {hotkey:?}");

            if let Some(handler) = self.hotkeys.get(&hotkey) {
                return Some(AppMessage::Action(handler()));
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
