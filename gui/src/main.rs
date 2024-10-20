#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use config::{
    workdir::{create_config_dir, create_workdir},
    Config,
};
use iced::{
    futures::{SinkExt, Stream},
    keyboard::{on_key_press, Key},
    stream,
    widget::{
        center,
        text_editor::{self, Content},
    },
    Element, Settings, Subscription, Task,
};
use log::{debug, info};
use state::{
    ActionBrocker, ActionResult, ActionWrapper, DocumentActor, FileActor, PaneActor, State,
};
use tokio::sync::mpsc::{self, channel, Sender};

use std::{collections::HashMap, path::PathBuf};

use crate::util::get_file_name;

use plugin::{ExamplePlugin, Plugin, PluginHost, PluginId, PluginInfo};

use theming::{catalog::Catalog, Theme};
use widget::{
    container::background,
    pane::pane_stack::{self, pane_stack},
};

use core::{
    action::{Action, FileAction, GenericAction, PaneAction},
    document::{DocumentHandler, DocumentId, DocumentStore},
    pane::{Pane, PaneModel},
    smol_str::SmolStr,
    value::Value,
    HotKey, Modifiers,
};

type HotKeyHandler = dyn Fn(&State) -> AppMessage;

static DEFAULT_THEME: &str = "core.light";
static APP_ICON: &[u8] = include_bytes!("../../contrib/icon.ico");

pub struct App {
    state: State,
    brocker_tx: Sender<ActionWrapper>,
    completition_tx: tokio::sync::broadcast::Sender<ActionResult>,
    plugin_host: PluginHost,
    hotkeys: HashMap<HotKey, Box<HotKeyHandler>>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    LoadPlugin(PluginId, bool),
    OpenedFile(Result<(PathBuf, String), ()>),
    GenericAction(GenericAction),
    Action(Action),
    OpenDirectory(PathBuf),
    TextEditorAction(text_editor::Action, DocumentId),
    OnKeyPress(Key, iced::keyboard::Modifiers),
    None,
}

impl App {
    fn new(config: Config) -> (Self, Task<AppMessage>) {
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

        let mut tasks = Vec::new();

        // Actors

        let (document_tx, document_rx) = channel(10);
        let (file_tx, file_rx) = channel(10);
        let (pane_tx, pane_rx) = channel(10);
        let (brocker_tx, brocker_rx) = channel(10);
        let (completition_tx, _) = tokio::sync::broadcast::channel(10);

        let mut document_actor = DocumentActor::new(document_rx, brocker_tx.clone());
        let mut pane_actor = PaneActor::new(pane_rx, brocker_tx.clone());
        let mut file_actor = FileActor::new(file_rx, brocker_tx.clone());

        let mut brocker = ActionBrocker::new(brocker_rx)
            .document_actor(document_tx.clone())
            .file_actor(file_tx.clone())
            .pane_actor(pane_tx.clone());

        tokio::spawn(async move { brocker.run().await });

        tokio::spawn(async move { document_actor.run().await });
        tokio::spawn(async move { pane_actor.run().await });
        tokio::spawn(async move { file_actor.run().await });

        let mut panes = PaneModel::new();
        {
            let id = panes.add(Pane::NewDocument);
            panes.open(&id);
        }

        let state = State {
            documents: DocumentStore::new(),
            panes,
            themes: Catalog::new(),
            config,
        };

        let mut app = Self {
            state,
            brocker_tx,
            completition_tx,
            plugin_host,
            hotkeys: HashMap::new(),
        };

        // Ctrl-o open file
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'o',
            },
            |_: &State| AppMessage::Action(Action::new(FileAction::PickFile)),
        );

        // Ctrl-t open new document tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 't',
            },
            |_: &State| AppMessage::Action(Action::new(PaneAction::Add(Pane::NewDocument, None))),
        );

        // Ctrl-w close open tab
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: 'w',
            },
            |state: &State| {
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
            |_state: &State| AppMessage::Action(Action::new(PaneAction::Add(Pane::Buffer, None))),
        );

        // Ctrl-, open config viewer pane
        app.add_hotkey(
            HotKey {
                modifiers: Modifiers::Ctrl,
                key: ',',
            },
            |_state: &State| AppMessage::Action(Action::new(PaneAction::Add(Pane::Config, None))),
        );

        for id in app.plugin_host.get_plugin_ids() {
            let task = Task::done(AppMessage::LoadPlugin(id.clone(), true));
            tasks.push(task);
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
        (app, Task::batch(tasks))
    }

    fn add_hotkey<F>(&mut self, hotkey: HotKey, func: F)
    where
        F: Fn(&State) -> AppMessage + 'static,
    {
        info!("Added hotkey {hotkey:?}");
        self.hotkeys.insert(hotkey, Box::new(func));
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn perform_action(&mut self, action: GenericAction) -> Task<AppMessage> {
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

            AppMessage::Action(action) => {
                info!("Update. Processing action in update");
                let action = self.plugin_host.process_action(&self.state, action);
                let mut tasks = Vec::new();
                for generic in action.iter() {
                    tasks.push(self.perform_action(generic.clone()));
                }

                return Task::batch(tasks);
            }

            AppMessage::GenericAction(action) => return self.perform_action(action),

            AppMessage::OnKeyPress(key, modifiers) => {
                if let Some(message) = self.on_key_press(key, modifiers) {
                    return Task::done(message);
                }
            }

            AppMessage::LoadPlugin(id, load) => {
                if load {
                    self.plugin_host.load_plugin(&id);
                } else {
                    self.plugin_host.unload_plugin(&id);
                }
            }

            AppMessage::TextEditorAction(action, document) => {
                if let Some(handler) = self.state.documents.get_mut(&document) {
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

                    let doc_id = self.state.documents.add(handler);
                    let pane = Pane::Editor(doc_id);

                    // If opened pane is NewDocument, replace it with Editor pane
                    // otherwise add new one with Editor
                    if let Some(&Pane::NewDocument) = self.state.panes.get_open() {
                        self.state.panes.replace(
                            &self.state.panes.get_open_id().cloned().unwrap_or(0usize),
                            pane,
                        );
                    } else {
                        let pane_id = self.state.panes.add(pane);
                        self.state.panes.open(&pane_id);
                    }
                }
            }

            AppMessage::OpenDirectory(path) => {
                if path.is_dir() {
                    let path: PathBuf = path.canonicalize().unwrap_or_default();

                    self.state.config.insert(
                        "system",
                        "workdir",
                        Value::String(SmolStr::new(path.to_str().unwrap_or_default())),
                    );
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<AppMessage, Theme> {
        let (tx, mut rx) = mpsc::channel(1);
        let get_model_action = ActionWrapper::new(GenericAction::Pane(PaneAction::GetModel(tx)));
        let _ = self.brocker_tx.try_send(get_model_action);
        if let Ok(Some(model)) = rx.try_recv() {
            let pane_stack = pane_stack(model).map(|message| match message {
                pane_stack::Message::OpenPane(id) => {
                    AppMessage::Action(Action::new(PaneAction::Open(id)))
                }
                pane_stack::Message::ClosePane(id) => {
                    AppMessage::Action(Action::new(PaneAction::Close(id)))
                }
                pane_stack::Message::NewPane(pane) => {
                    AppMessage::Action(Action::new(PaneAction::Add(pane, None)))
                }
                pane_stack::Message::NewDocument(_message) => todo!(),
                pane_stack::Message::TextEditor(_, _message) => todo!(),
                pane_stack::Message::None => AppMessage::None,
            });
            pane_stack
        } else {
            background(center("Can't load PaneModel")).into()
        }
    }

    fn theme(&self) -> Theme {
        (*self.state.get_theme()).clone()
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

            if let Some(func) = self.hotkeys.get(&hotkey) {
                return Some(func(&self.state));
            }
        }
        None
    }
}

fn main() -> iced::Result {
    env_logger::init();

    let mut config = Config::new();

    // Initializing workdir. Default is ~/strelka
    let workdir_path = if let Ok(path) = create_workdir() {
        path
    } else {
        panic!("Can't create workdir")
    };

    config.insert(
        "system",
        "workdir",
        Value::String(SmolStr::new(workdir_path.to_str().unwrap())),
    );

    // Initializing config directory. Default is ~/strelka/.config
    let config_dir_path = if let Ok(path) = create_config_dir(&workdir_path) {
        path
    } else {
        panic!("Can't create config directory")
    };

    config.insert(
        "system",
        "config_dir",
        Value::String(SmolStr::new(config_dir_path.to_str().unwrap())),
    );

    // Path to system config file
    let system_config_path = {
        let mut a = config_dir_path.clone();
        a.push("system.toml");
        a
    };

    // Default config which used when config from file doesn't loaded
    let mut default_config = Config::new();
    default_config.insert(
        "system",
        "theme",
        Value::String(SmolStr::new(DEFAULT_THEME)),
    );

    // Loading system config from file or initializing it with default one
    let system_config =
        Config::load_or_create_default(&system_config_path, default_config).unwrap();
    config.merge(system_config);

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
        .run_with(move || App::new(config))
}
