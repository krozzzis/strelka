mod camera;
mod plugin;
mod scene;

use iced::{executor, Command};
use iced::{Application, Element, Settings};
use iced::{Color, Point, Theme};

use std::sync::Arc;

use crate::{
    plugin::{ExamplePlugin, Plugin, PluginHost},
    scene::{Spline, *},
};

#[derive(Default)]
pub struct App {
    scene: Scene,
    debug: bool,
    plugin_host: PluginHost,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SendPluginAction {
        name: String,
        action: Arc<plugin::Action>,
    },
    SetDebug(bool),
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut host = PluginHost::new();
        host.register_plugin(
            "core.example",
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );

        let app = Self {
            scene: Scene::new()
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
                }),
            debug: false,
            plugin_host: host,
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

            AppMessage::SendPluginAction { name, action } => {
                self.plugin_host.send_action(name, action);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let editor: Canvas<AppMessage> = canvas(&self.scene)
            .on_plugin_action(|a, b| AppMessage::SendPluginAction { name: a, action: b });

        editor.into()
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
