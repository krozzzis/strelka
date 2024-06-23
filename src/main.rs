use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::path;
use iced::widget::canvas::{self, event::Status, Path, Cache, Canvas, Event, Frame, Geometry, Program};
use iced::widget::{button, text, column, row};
use iced::Color;
use iced::Length;
use iced::{Point, Size};
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::{executor, keyboard, Command, Subscription, Vector};
use iced::{Alignment, Application, Element, Settings};

#[derive(Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f32,
    pub bb_start: Point,
    pub bb_size: Size,
}

impl Stroke {
    pub fn calculate_bb(&mut self) {
        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;

        for pos in &self.points[1..] {
            max_x = max_x.max(pos.x);
            max_y = max_y.max(pos.y);
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
        }

        self.bb_start = Point::new(min_x, min_y);
        self.bb_size = Size::new(max_x - min_x, max_y - min_y);
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: Color::BLACK,
            width: 4.0,
            bb_start: Point::ORIGIN,
            bb_size: Size::ZERO,
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
    tool: FieldTool,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    AddStroke(Stroke),
    AddPointToStroke(Point),
    ChangeStrokeColor(Color),
    EndStroke,
    RemoveLastStroke,
    ClearStrokes,
    ChangeTool(FieldTool),
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("P3")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::ChangeTool(x) => {
                self.tool = x;
                self.canvas_state.cache.clear();
            }

            Self::Message::AddStroke(x) => {
                self.strokes.push(x);
                self.canvas_state.cache.clear();
            }

            Self::Message::AddPointToStroke(x) => {
                self.canvas_state.stroke.points.push(x);
                self.canvas_state.stroke.calculate_bb();
            }

            Self::Message::EndStroke => {
                if !self.canvas_state.stroke.points.is_empty() {
                    println!("Stroke len: {} points", self.canvas_state.stroke.points.len());
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
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let canvas = Canvas::new(Field {
            strokes: &self.strokes,
            canvas_state: &self.canvas_state,
            tool: self.tool,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let tool_text = text(format!("Current tool: {}", match self.tool {
            FieldTool::Pen => "Pen",
            FieldTool::Select => "Select",
        }));

        let tools = row![
            tool_text.size(18),
            button("Pen (1)").on_press(AppMessage::ChangeTool(FieldTool::Pen)),
            button("Select (2)").on_press(AppMessage::ChangeTool(FieldTool::Select)),
        ];

        let colors = row![
            button("Black").on_press(AppMessage::ChangeStrokeColor(Color::new(
                0.1, 0.1, 0.1, 1.0
            ))),
            button("Red").on_press(AppMessage::ChangeStrokeColor(Color::new(
                1.0, 0.1, 0.1, 1.0
            ))),
            button("Blue").on_press(AppMessage::ChangeStrokeColor(Color::new(
                0.1, 0.1, 1.0, 1.0
            ))),
            button("Yellow").on_press(AppMessage::ChangeStrokeColor(Color::new(
                1.0, 1.0, 0.1, 1.0
            ))),
        ];

        column![tools, colors, canvas]
            .padding(10)
            .align_items(Alignment::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("z") => {
                if modifiers.control() {
                    Some(AppMessage::RemoveLastStroke)
                } else {
                    None
                }
            }

            keyboard::Key::Character("1") => Some(AppMessage::ChangeTool(FieldTool::Pen)),
            keyboard::Key::Character("2") => Some(AppMessage::ChangeTool(FieldTool::Select)),
            
            _ => None,
        })
    }
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

struct Field<'a> {
    canvas_state: &'a CanvasState,
    strokes: &'a [Stroke],
    tool: FieldTool,
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

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
