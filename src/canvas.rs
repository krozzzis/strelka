use super::stroke::*;
use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::{
    self, event::Status, path, path::Path, Cache, Event, Frame, Geometry, Program,
};
use iced::Rectangle;
use iced::{Color, Renderer, Theme, Vector};

#[derive(Default)]
pub struct CanvasState {
    pub cache: Cache,
}


pub struct SceneEditorProgram<'a> {
    pub canvas_state: &'a CanvasState,
    pub strokes: Scene,
}

impl<'a> Program<EditorMessage> for SceneEditorProgram<'a> {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let content = self
            .canvas_state
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {});

        let mut frame = Frame::new(renderer, bounds.size());

        vec![content, frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
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
        (Status::Ignored, None)
    }
}
