use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::{self, event::Status, Canvas, Cache, Event, Frame, Geometry, Program};
use iced::widget::canvas::path;
use iced::Color;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;
use iced::{Element, Sandbox, Settings};
use rand::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: Color,
}

#[derive(Default)]
struct CanvasState {
    cache: Cache,
}

#[derive(Default)]
pub struct App {
    canvas_state: CanvasState,
    strokes: Vec<Stroke>,
}

#[derive(Debug, Clone)]
pub enum Mess {
    AddStroke(Stroke),
    ClearStrokes,
}

impl Sandbox for App {
    type Message = Mess;

    fn new() -> Self {
        App::default()
    }

    fn title(&self) -> String {
        String::from("P3")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::AddStroke(x) => {
                self.strokes.push(x);
            }

            Self::Message::ClearStrokes => {
                self.strokes.clear();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let canvas: Element<Stroke> = Canvas::new(Field {
            strokes: &self.strokes,
            canvas_state: &self.canvas_state,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        canvas.map(Self::Message::AddStroke)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FieldMouse {
    #[default]
    Idle,
    Stroking,
}

#[derive(Default, Debug, Clone)]
pub struct FieldState {
    mouse: FieldMouse,
    points: Vec<Point>,
    color: Color,
}

struct Field<'a> {
    canvas_state: &'a CanvasState,
    strokes: &'a [Stroke],
}

// Then, we implement the `Program` trait
impl<'a> Program<Stroke> for Field<'a> {
    type State = FieldState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(renderer, bounds.size());

        for i in 0..self.strokes.len() {
            let stroke = &self.strokes[i];
            if stroke.points.len() > 0 {
                let mut builder = path::Builder::new();

                builder.move_to(stroke.points[0]);
                for pos in &stroke.points {
                    builder.line_to(*pos);
                }
                frame.stroke(&builder.build(), canvas::Stroke {
                    width: 2.0,
                    style: canvas::Style::Solid(stroke.color),
                    line_cap: canvas::LineCap::Round,
                    line_join: canvas::LineJoin::Round,
                    line_dash: canvas::LineDash::default(),
                });
            }
        }

        //for v in self.poses {
        //    // We create a `Path` representing a simple circle
        //    let circle = Path::circle(Point::new(v.x, v.y), circle_radius);
        //
        //    // And fill it with some color
        //    frame.fill(&circle, Color::BLACK);
        //}

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(&self, _state: &Self::State, bounds: Rectangle, cursor: Cursor) -> Interaction {
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
    ) -> (Status, Option<Stroke>) {
        if let Some(cursor_pos) = cursor.position_in(bounds) {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match state.mouse {
                            FieldMouse::Idle => {
                                state.mouse = FieldMouse::Stroking;
                                let mut rng = rand::thread_rng();
                                state.color = Color::new(
                                    rng.gen_range(0.0..1.0), 
                                    rng.gen_range(0.0..1.0), 
                                    rng.gen_range(0.0..1.0), 
                                    1.0
                                    );
                                state.points.push(cursor_pos);
                            },
                            FieldMouse::Stroking => {
                                state.points.push(cursor_pos);
                            },
                        }
                        (Status::Captured, None)
                    },

                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        match state.mouse {
                            FieldMouse::Idle => {
                                state.points.push(cursor_pos);
                            },
                            FieldMouse::Stroking => {
                                state.mouse = FieldMouse::Idle;
                                state.points.push(cursor_pos);
                            },
                        }
                        state.points.clear();
                        (Status::Captured, Some(Stroke {
                            points: state.points.clone(),
                            color: state.color,
                        }))
                    },

                    mouse::Event::CursorMoved { position: cursor_pos } => {
                        match state.mouse {
                            FieldMouse::Stroking => {
                                state.points.push(cursor_pos);
                            },
                            _ => {},
                        }
                        (Status::Captured, Some(Stroke {
                            points: state.points.clone(),
                            color: state.color,
                        }))
                    },

                    _ => (Status::Ignored, None),
                },

                _ => (Status::Ignored, None),
            }
        } else {
            (Status::Ignored, None)
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
