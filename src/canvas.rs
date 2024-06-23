use iced::Rectangle;
use iced::{Theme, Renderer, Vector, Color};
use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::{self, Event, Cache, Program, Frame, Geometry, path, path::Path, event::Status};
use crate::AppMessage;
use super::stroke::*;

#[derive(Default)]
pub struct CanvasState {
    pub cache: Cache,
    pub stroke: Stroke,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FieldMouse {
    #[default]
    Idle,
    Stroking,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FieldTool {
    #[default]
    Pen,
    Select,
}

#[derive(Default, Debug, Clone)]
pub struct FieldState {
    mouse: FieldMouse,
    selected_stroke: usize,
}

pub struct Field<'a> {
    pub canvas_state: &'a CanvasState,
    pub strokes: &'a [Stroke],
    pub tool: FieldTool,
}

fn select_tool(
    field: &Field,
    state: &mut FieldState,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor,
) -> (Status, Option<AppMessage>) {
    if let Some(cursor_pos) = cursor.position_in(bounds) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    state.selected_stroke = 0;
                    for i in 0..field.strokes.len() {
                        let stroke = &field.strokes[i];
                        let bb = Rectangle::new(
                            stroke.bb_start + Vector::new(bounds.position().x, bounds.position().y), 
                            stroke.bb_size);
                        if let true = cursor.is_over(bb) {
                            state.selected_stroke = i + 1;
                        }
                    }
                    field.canvas_state.cache.clear();
                },

                _ => {},
            },

            _ => {},
        }
    }
    (Status::Ignored, None)
}

fn pen_tool(
    field: &Field,
    state: &mut FieldState,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor,
) -> (Status, Option<AppMessage>) {
    match event {
        Event::Mouse(mouse_event) => match mouse_event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    match state.mouse {
                        FieldMouse::Idle => {
                            state.mouse = FieldMouse::Stroking;
                            (Status::Captured, None)
                        }
                        FieldMouse::Stroking => (
                            Status::Captured,
                            Some(AppMessage::AddPointToStroke(cursor_pos)),
                        ),
                    }
                } else {
                    (Status::Ignored, None)
                }
            }

            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                match state.mouse {
                    FieldMouse::Idle => {}
                    FieldMouse::Stroking => {
                        state.mouse = FieldMouse::Idle;
                    }
                }
                (Status::Captured, Some(AppMessage::EndStroke))
            }

            mouse::Event::CursorMoved { .. } => match state.mouse {
                FieldMouse::Stroking => {
                    if let Some(cursor_pos) = cursor.position_in(bounds) {
                        (
                            Status::Captured,
                            Some(AppMessage::AddPointToStroke(cursor_pos)),
                        )
                    } else {
                        (Status::Captured, Some(AppMessage::EndStroke))
                    }
                }
                FieldMouse::Idle => (Status::Captured, None),
            },

            _ => (Status::Ignored, None),
        },

        _ => (Status::Ignored, None),
    }
}

impl<'a> Program<AppMessage> for Field<'a> {
    type State = FieldState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let draw_stroke = |stroke: &Stroke, id: usize, frame: &mut Frame| {
            if !stroke.points.is_empty() {
                let mut builder = path::Builder::new();

                builder.move_to(stroke.points[0]);
                for pos in &stroke.points {
                    builder.line_to(*pos);
                }

                frame.stroke(
                    &builder.build(),
                    canvas::Stroke {
                        width: stroke.width,
                        style: canvas::Style::Solid(stroke.color),
                        line_cap: canvas::LineCap::Round,
                        line_join: canvas::LineJoin::Round,
                        line_dash: canvas::LineDash::default(),
                    },
                );

                if let FieldTool::Select = self.tool {
                    if id == state.selected_stroke {
                        frame.stroke(
                            &Path::rectangle(stroke.bb_start, stroke.bb_size),
                            canvas::Stroke {
                                width: 3.0,
                                style: canvas::Style::Solid(Color::BLACK),
                                line_cap: canvas::LineCap::Square,
                                line_join: canvas::LineJoin::Miter,
                                line_dash: canvas::LineDash::default(),
                            },
                        );
                    }
                }
            }
        };

        let content = self
            .canvas_state
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                for i in 0..self.strokes.len() {
                    let stroke = &self.strokes[i];
                    draw_stroke(stroke, i + 1, frame);
                }
            });

        let mut frame = Frame::new(renderer, bounds.size());

        draw_stroke(&self.canvas_state.stroke, self.strokes.len(), &mut frame);

        vec![content, frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(bounds) {
            match self.tool {
                FieldTool::Pen => mouse::Interaction::Crosshair,
                FieldTool::Select => mouse::Interaction::default(),
            }
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
    ) -> (Status, Option<AppMessage>) {
        match self.tool {
            FieldTool::Pen => pen_tool(self, state, event, bounds, cursor),
            FieldTool::Select => select_tool(self, state, event, bounds, cursor),
            _ => (Status::Ignored, None),
        }
    }
}
