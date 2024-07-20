mod camera;
mod canvas;
mod plugin;

use iced::{
    border::Radius,
    keyboard::{self, key::Named, Key},
    widget::{
        button, center,
        container::{self, Style},
        mouse_area, opaque, row, stack, text, text_input, Container,
    },
    Length, Subscription,
};
use iced::{Color, Element, Point, Settings};

use std::sync::Arc;

use crate::{
    canvas::{canvas, Canvas, Rectangle, Scene, Spline},
    plugin::{plugin_list, ExamplePlugin, Plugin, PluginHost, PluginId, PluginInfo},
};

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
            debug: false,
        }
    }
}

impl App {
    fn title(&self) -> String {
        String::from("p3")
    }

    fn update(&mut self, message: AppMessage) {
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
        }
    }

    fn view(&self) -> Element<AppMessage> {
        let canvas_renderer: Canvas<AppMessage> = canvas(&self.scene)
            .on_plugin_action(|a, b| AppMessage::SendPluginAction { id: a, action: b });

        let primary_screen = Container::new(canvas_renderer).width(Length::Fill);

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
        .centered()
        .run()
}
