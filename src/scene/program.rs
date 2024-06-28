use crate::scene::Scene;
use crate::{stroke::*, EditorMessage, SceneRenderer};

use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::{event::Status, Cache, Event, Geometry, Program};
use iced::{Rectangle, Renderer, Theme};

#[derive(Default)]
pub struct CanvasState {
    pub cache: Cache,
    pub pending_stroke: Stroke,
}

pub struct CanvasProgram<'a> {
    pub state: &'a CanvasState,
    pub scene: &'a Scene,
}

impl<'a> CanvasProgram<'a> {
    pub fn new(state: &'a CanvasState, scene: &'a Scene) -> Self {
        Self { state, scene }
    }
}

impl<'a> Program<EditorMessage> for CanvasProgram<'a> {
    type State = bool;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let scene_renderer = SceneRenderer::new(self.scene);
        let geometry = scene_renderer.draw(renderer, bounds);

        let pending_scene = Scene {
            strokes: vec![self.state.pending_stroke.clone()],
        };
        let pending_renderer = SceneRenderer::new(&pending_scene);
        let pending_geometry = pending_renderer.draw(renderer, bounds);

        vec![geometry, pending_geometry]
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
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<EditorMessage>) {
        if let Some(local_pos) = cursor.position_in(bounds) {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        *state = true;
                        (
                            Status::Captured,
                            Some(EditorMessage::AddPointToPending(local_pos)),
                        )
                    }

                    mouse::Event::CursorMoved { .. } => {
                        if *state {
                            (
                                Status::Captured,
                                Some(EditorMessage::AddPointToPending(local_pos)),
                            )
                        } else {
                            (Status::Ignored, None)
                        }
                    }

                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        *state = false;
                        (Status::Captured, Some(EditorMessage::AddPendingToScene))
                    }

                    _ => (Status::Ignored, None),
                },

                _ => (Status::Ignored, None),
            }
        } else {
            (Status::Ignored, None)
        }
    }
}
