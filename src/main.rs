use iced::mouse::{self, Cursor, Interaction};
use iced::widget::canvas::{event::Status, Canvas, Event, Frame, Geometry, Path, Program};
use iced::Color;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;
use iced::{Element, Sandbox, Settings};

#[derive(Default)]
pub struct App {
    points: Vec<Vector>,
}

#[derive(Debug, Clone, Copy)]
pub enum Mess {
    AddPoint(Vector),
    ClearPoints,
}

impl Sandbox for App {
    type Message = Mess;

    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn title(&self) -> String {
        String::from("P3")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::AddPoint(x) => {
                self.points.push(x);
            }

            Self::Message::ClearPoints => {
                self.points.clear();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let canvas: Element<Vector> = Canvas::new(Circle {
            poses: &self.points,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        canvas.map(Self::Message::AddPoint)
    }
}

#[derive(Debug)]
struct Circle<'a> {
    poses: &'a [Vector],
}

// Then, we implement the `Program` trait
impl<'a> Program<Vector> for Circle<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(renderer, bounds.size());
        let circle_radius = 5.0;

        for v in self.poses {
            // We create a `Path` representing a simple circle
            let circle = Path::circle(Point::new(v.x, v.y), circle_radius);

            // And fill it with some color
            frame.fill(&circle, Color::BLACK);
        }

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(&self, _state: &(), bounds: Rectangle, cursor: Cursor) -> Interaction {
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
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Vector>) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(cursor_pos) = cursor.position_in(bounds) {
                        (
                            Status::Captured,
                            Some(Vector::new(cursor_pos.x, cursor_pos.y)),
                        )
                    } else {
                        (Status::Ignored, None)
                    }
                }

                _ => (Status::Ignored, None),
            },

            _ => (Status::Ignored, None),
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
