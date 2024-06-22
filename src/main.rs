use iced::mouse::{self, Cursor, Interaction};
use iced::widget::{row, column, button};
use iced::widget::canvas::path;
use iced::widget::canvas::{self, event::Status, Cache, Canvas, Event, Frame, Geometry, Program};
use iced::Color;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::{Alignment, Element, Sandbox, Settings};

#[derive(Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: Color,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: Color::BLACK,
        }
    }
}

#[derive(Default)]
struct CanvasState {
    cache: Cache,
    stroke: Stroke,
}

#[derive(Default)]
pub struct App {
    canvas_state: CanvasState,
    strokes: Vec<Stroke>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    AddStroke(Stroke),
    AddPointToStroke(Point),
    ChangeStrokeColor(Color),
    EndStroke,
    RemoveLastStroke,
    ClearStrokes,
}

impl Sandbox for App {
    type Message = AppMessage;

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
                self.canvas_state.cache.clear();
            }

            Self::Message::AddPointToStroke(x) => {
                self.canvas_state.stroke.points.push(x);
                self.canvas_state.cache.clear();
            }

            Self::Message::EndStroke => {
                if (!self.canvas_state.stroke.points.is_empty()) {
                    self.strokes.push(self.canvas_state.stroke.clone());
                    self.canvas_state.stroke.points.clear();
                    self.canvas_state.cache.clear();
                }
            }

            Self::Message::ChangeStrokeColor(x) => {
                self.canvas_state.stroke.color = x;
            }

            Self::Message::ClearStrokes => {
                self.strokes.clear();
                self.canvas_state.cache.clear();
            }

            Self::Message::RemoveLastStroke => {
                self.strokes.pop();
                self.canvas_state.cache.clear();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let canvas = Canvas::new(Field {
            strokes: &self.strokes,
            canvas_state: &self.canvas_state,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let colors = row![
            button("Black").on_press(AppMessage::ChangeStrokeColor(Color::new(0.1, 0.1, 0.1, 1.0))),
            button("Red").on_press(AppMessage::ChangeStrokeColor(Color::new(1.0, 0.1, 0.1, 1.0))),
            button("Blue").on_press(AppMessage::ChangeStrokeColor(Color::new(0.1, 0.1, 1.0, 1.0))),
            button("Yellow").on_press(AppMessage::ChangeStrokeColor(Color::new(1.0, 1.0, 0.1, 1.0))),
        ];

        column![colors, canvas]
        .padding(10)
        .align_items(Alignment::Center)
        .into()
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
}

struct Field<'a> {
    canvas_state: &'a CanvasState,
    strokes: &'a [Stroke],
}

// Then, we implement the `Program` trait
impl<'a> Program<AppMessage> for Field<'a> {
    type State = FieldState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let draw_stroke = |stroke: &Stroke, frame: &mut Frame| {
            if !stroke.points.is_empty() {
                let mut builder = path::Builder::new();

                builder.move_to(stroke.points[0]);
                for pos in &stroke.points {
                    builder.line_to(*pos);
                }

                frame.stroke(
                    &builder.build(),
                    canvas::Stroke {
                        width: 5.0,
                        style: canvas::Style::Solid(stroke.color),
                        line_cap: canvas::LineCap::Round,
                        line_join: canvas::LineJoin::Round,
                        line_dash: canvas::LineDash::default(),
                    },
                );
            }
        };

        let content = self
            .canvas_state
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                for i in 0..self.strokes.len() {
                    let stroke = &self.strokes[i];
                    draw_stroke(stroke, frame);
                }
            });

        let mut frame = Frame::new(renderer, bounds.size());

        draw_stroke(&self.canvas_state.stroke, &mut frame);

        vec![content, frame.into_geometry()]
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
    ) -> (Status, Option<AppMessage>) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Right) => {
                    if let Some(cursor_pos) = cursor.position_in(bounds) {
                        (Status::Captured, Some(AppMessage::RemoveLastStroke))
                    } else {
                        (Status::Ignored, None)
                    }
                },

                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(cursor_pos) = cursor.position_in(bounds) {
                        match state.mouse {
                            FieldMouse::Idle => {
                                state.mouse = FieldMouse::Stroking;
                                (
                                    Status::Captured,
                                    None,
                                )
                            }
                            FieldMouse::Stroking => (
                                Status::Captured,
                                Some(AppMessage::AddPointToStroke(cursor_pos)),
                            ),
                        }
                    } else {
                        (Status::Ignored, None)
                    }
                },

                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    match state.mouse {
                        FieldMouse::Idle => {}
                        FieldMouse::Stroking => {
                            state.mouse = FieldMouse::Idle;
                        }
                    }
                    (Status::Captured, Some(AppMessage::EndStroke))
                }

                mouse::Event::CursorMoved { .. } => {
                    match state.mouse {
                        FieldMouse::Stroking => {
                            if let Some(cursor_pos) = cursor.position_in(bounds) {
                                (
                                    Status::Captured,
                                    Some(AppMessage::AddPointToStroke(cursor_pos)),
                                )
                            } else {
                                (Status::Captured, Some(AppMessage::EndStroke))
                            }
                        },
                        FieldMouse::Idle => (Status::Captured, None),
                    }
                },

                _ => (Status::Ignored, None),
            },

            _ => (Status::Ignored, None),
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
