use std::sync::Arc;

use crate::{
    camera::Camera,
    canvas::{EditorMessage, Scene},
    plugin,
};

use iced::{
    keyboard::{key::Named, Key},
    widget::canvas::{event::Status, Cache, Event, Geometry, Program},
};
use iced::{
    mouse::{self, Cursor, Interaction},
    widget::canvas::Frame,
};
use iced::{Rectangle, Renderer, Theme};

#[derive(Default)]
pub struct CanvasProgramState {
    pub camera: Camera,
}

pub struct CanvasProgram<'a> {
    pub cache: &'a Cache,
    pub scene: &'a Scene,
}

impl<'a> Program<EditorMessage> for CanvasProgram<'a> {
    type State = CanvasProgramState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let cached_geometry = self
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                self.scene.draw(frame, &state.camera);
            });

        vec![cached_geometry]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<EditorMessage>) {
        match event {
            Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed {
                    key: Key::Named(Named::F1),
                    location: _,
                    modifiers: _,
                    text: _,
                } => {
                    return (
                        Status::Captured,
                        Some(EditorMessage::SendPluginAction {
                            plugin: String::from("core.example"),
                            action: Arc::new(plugin::Action::new(
                                String::from("say"),
                                String::from("Hello"),
                            )),
                        }),
                    )
                }

                _ => {}
            },

            _ => {}
        }
        (Status::Ignored, None)
    }
}
