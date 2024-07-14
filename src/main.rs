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
    plugin::{ExamplePlugin, Plugin, PluginHost},
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
        name: String,
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
            Box::new(ExamplePlugin {}) as Box<dyn Plugin>,
        );
        plugin_host.register_plugin(
            "core.example2",
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

            AppMessage::LoadPlugin(name, load) => {
                if load {
                    self.plugin_host.load_plugin(&name);
                } else {
                    self.plugin_host.unload_plugin(&name);
                }
            }

            AppMessage::SendPluginAction { name, action } => {
                self.plugin_host.send_action(name, action);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut plugin_entries = Vec::new();
        for name in self.plugin_host.get_plugin_names() {
            let load = self.plugin_host.is_plugin_loaded(&name);

            if let Some(load) = load {
                let entry = row![
                    text(name.clone()).width(Length::Fixed(128.0)),
                    Toggler::new(
                        if load {
                            "Loaded".to_string()
                        } else {
                            "Unloaded".to_string()
                        },
                        load,
                        move |state| AppMessage::LoadPlugin(name.clone(), state)
                    )
                    .width(Length::Shrink)
                ]
                .width(Length::Shrink)
                .spacing(4.0);

                plugin_entries.push(Container::new(entry).padding(Padding::from(8.0)).into());
            }
        }
        let plugins_list = Container::new(Column::from_vec(vec![
            text("Plugins:").into(),
            Scrollable::new(Column::from_vec(plugin_entries)).into(),
        ]))
        .padding(8.0);

        let canvas_renderer: Canvas<AppMessage> = canvas(&self.scene)
            .on_plugin_action(|a, b| AppMessage::SendPluginAction { name: a, action: b });

        row![
            plugins_list.width(Length::Fixed(280.0)),
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
