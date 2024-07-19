mod camera;
mod canvas;
mod plugin;

use iced::{
    executor,
    widget::{row, text, Column, Container, Scrollable, Toggler},
    Command, Length, Padding,
};
use iced::{Application, Element, Settings};
use iced::{Color, Point, Theme};

use std::sync::Arc;

use crate::{
    canvas::{canvas, Canvas, Rectangle, Scene, Spline},
    plugin::{ExamplePlugin, Plugin, PluginHost, PluginId, PluginInfo, PluginStatus},
};

#[derive(Default)]
pub struct App {
    debug: bool,
    scene: Scene,
    plugin_host: PluginHost,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SendPluginAction {
        id: PluginId,
        action: Arc<plugin::Action>,
    },
    LoadPlugin(String, bool),
    SetDebug(bool),
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut plugin_host = PluginHost::new();
        plugin_host.register_plugin(
            "core.example",
            PluginInfo::new().name("ExamplePlugin"),
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );
        plugin_host.register_plugin(
            "core.example2",
            PluginInfo::new().name("ExamplePlugin2"),
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

        let app = Self {
            scene,
            plugin_host,
            debug: false,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("p3")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
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
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut plugin_entries = Vec::new();
        let mut plugin_ids = self.plugin_host.get_plugin_ids();
        plugin_ids.sort_unstable();
        for id in plugin_ids {
            let plugin_state = self.plugin_host.get_plugin_status(&id);
            let loaded: bool = match plugin_state {
                Some(PluginStatus::Loaded) => true,
                Some(PluginStatus::Unloaded) => false,
                _ => false,
            };

            let entry = row![
                text(id.clone()).width(Length::Fixed(128.0)),
                Toggler::new(
                    if loaded {
                        "Loaded".to_string()
                    } else {
                        "Unloaded".to_string()
                    },
                    loaded,
                    move |state| AppMessage::LoadPlugin(id.clone(), state)
                )
                .width(Length::Shrink)
            ]
            .spacing(8.0);

            plugin_entries.push(
                Container::new(entry)
                    .padding(Padding::from(8.0))
                    .width(Length::Shrink)
                    .into(),
            );
        }
        let plugins_list = Container::new(Column::from_vec(vec![
            text("Plugins:").into(),
            Scrollable::new(Column::from_vec(plugin_entries)).into(),
        ]))
        .padding(8.0);

        let canvas_renderer: Canvas<AppMessage> = canvas(&self.scene)
            .on_plugin_action(|a, b| AppMessage::SendPluginAction { id: a, action: b });

        row![
            plugins_list.width(Length::Fixed(300.0)),
            Container::new(canvas_renderer).width(Length::Fill)
        ]
        .spacing(8.0)
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
