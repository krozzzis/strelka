mod scene;
mod stroke;

use iced::{executor, keyboard, Command, Subscription};
use iced::{Application, Element, Settings};
use iced::{Color, Point, Theme};

use scene::*;

use crate::stroke::Stroke;

#[derive(Default)]
pub struct App {
    scene: Scene,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    AddObject(Stroke),
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            scene: Scene::new(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("P3")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::AddObject(stroke) => {
                self.scene.add_object(stroke);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let editor: SceneEditor<AppMessage> =
            scene_editor(&self.scene).on_added_object(AppMessage::AddObject);
        editor.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("z") => None,

            _ => None,
        })
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
