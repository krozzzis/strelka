#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{AppConfig, GuiConfig};
use iced::{
    futures::{SinkExt, Stream},
    keyboard::{on_key_press, Key},
    stream,
    widget::{
        center,
        text_editor::{self},
    },
    Element, Settings, Subscription, Task,
};
use log::{debug, info, warn};
use state::{
    actors::{DocumentActor, FileActor, PaneActor, PluginHostActor, ThemeActor},
    ActionBrocker,
};
use tokio::sync;
use tokio::sync::mpsc::{self, channel};

use std::collections::HashMap;

use plugin::{ExamplePlugin, Plugin, PluginHost, PluginInfo};

use theming::Theme;
use widget::{
    container::background,
    pane::pane_stack::{self, pane_stack},
};

use core::{
    action::{
        Action, ActionResult, ActionWrapper, FileAction, IntoAction, Message, PaneAction,
        ThemeAction,
    },
    document::DocumentId,
    pane::Pane,
    smol_str::SmolStr,
    HotKey, Modifiers,
};

static DEFAULT_THEME: &str = "core.light";
static APP_ICON: &[u8] = include_bytes!("../../contrib/icon.ico");

pub struct App {
    config: AppConfig,
    brocker_tx: sync::mpsc::Sender<ActionWrapper>,
    completition_tx: sync::broadcast::Sender<ActionResult>,
    hotkeys: HashMap<HotKey, Action>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Action(Action),
    TextEditorAction(text_editor::Action, DocumentId),
    OnKeyPress(Key, iced::keyboard::Modifiers),
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

        let (completition_tx, _) = tokio::sync::broadcast::channel(10);

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
            },
        };

        let mut app = Self {
            config,
            brocker_tx,
            completition_tx,
            hotkeys: HashMap::new(),
        };

        // Ctrl-d run plugin's message
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'd',
            },
            Message {
                destination: "core.example".to_string(),
                kind: "test".to_string(),
                payload: None,
            },
        );

        // Ctrl-o open file
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'o',
            },
            FileAction::PickFile,
        );

        // Ctrl-t open new document tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 't',
            },
            PaneAction::Add(Pane::NewDocument, None),
        );

        // Ctrl-b open experimental buffer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'b',
            },
            PaneAction::Add(Pane::Buffer, None),
        );

        // Ctrl-, open config viewer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: ',',
            },
            PaneAction::Add(Pane::Config, None),
        );

        // Ctrl-, open config viewer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'm',
            },
            ThemeAction::MakeIndex,
        );
        {
            let task = Task::done(AppMessage::Action(ThemeAction::MakeIndex.into_action()));
            startup_tasks.push(task);
        }

        // Apply theme
        // let theme = if let Some(Value::String(id)) = app.state.config.get("system", "theme") {
        //     id
        // } else {
        //     SmolStr::new(DEFAULT_THEME)
        // };
        // let apply_theme = Task::perform(async move { theme }, AppMessage::LoadTheme);
        // tasks.push(apply_theme);

        info!("App constructor done");
        (app, Task::batch(startup_tasks))
    }

    fn add_hotkey(&mut self, hotkey: HotKey, action: impl IntoAction) {
        info!("Added hotkey {hotkey:?}");
        self.hotkeys.insert(hotkey, action.into_action());
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn perform_action(&mut self, action: Action) -> Task<AppMessage> {
        let wrapper = ActionWrapper::new(action).notify(self.completition_tx.clone());
        let brocker_tx = self.brocker_tx.clone();
        Task::perform(
            async move {
                let _ = brocker_tx.send(wrapper).await;
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

            AppMessage::TextEditorAction(_action, _document) => {
                todo!()
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<AppMessage, Theme> {
        let (tx, mut rx) = mpsc::channel(1);
        let get_model_action = ActionWrapper::new(Action::Pane(PaneAction::GetModel(tx)));
        let _ = self.brocker_tx.blocking_send(get_model_action);
        if let Some(Some(model)) = rx.blocking_recv() {
            info!("View. Loaded PaneModel");
            let pane_stack = pane_stack(model).map(|message| match message {
                pane_stack::Message::OpenPane(id) => {
                    AppMessage::Action(PaneAction::Open(id).into_action())
                }
                pane_stack::Message::ClosePane(id) => {
                    AppMessage::Action(PaneAction::Close(id).into_action())
                }
                pane_stack::Message::NewPane(pane) => {
                    AppMessage::Action(PaneAction::Add(pane, None).into_action())
                }
                pane_stack::Message::NewDocument(_message) => todo!(),
                pane_stack::Message::TextEditor(_, _message) => todo!(),
                pane_stack::Message::None => AppMessage::None,
            });
            pane_stack
        } else {
            warn!("View. Can't load PaneModel");
            background(center("Can't load PaneModel")).into()
        }
    }

    fn theme(&self) -> Theme {
        self.config.gui.theme.clone()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        info!("Creating subscriptions");
        let completition_listener =
            Subscription::run_with_id(1, App::complition_stream(self.completition_tx.subscribe()));
        let key_press_listener =
            on_key_press(|key, modifiers| Some(AppMessage::OnKeyPress(key, modifiers)));

        Subscription::batch([completition_listener, key_press_listener])
    }

    fn complition_stream(
        mut rx: tokio::sync::broadcast::Receiver<ActionResult>,
    ) -> impl Stream<Item = AppMessage> {
        info!("Completition listener started");
        stream::channel(100, |mut output| async move {
            info!("Completition listener thread");
            while let Ok(_result) = rx.recv().await {
                info!("Receive complete notificaton");
                let _ = output.send(AppMessage::None).await;
            }
        })
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

            if let Some(action) = self.hotkeys.get(&hotkey) {
                return Some(AppMessage::Action(action.clone()));
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
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .window(iced::window::Settings {
            icon: iced::window::icon::from_file_data(APP_ICON, None).ok(),
            ..Default::default()
        })
        .centered()
        .run_with(App::new)
}
