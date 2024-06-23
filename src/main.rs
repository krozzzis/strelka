mod stroke;
mod canvas;

use iced::widget::{button, text, column, row};
use iced::Color;
use iced::Length;
use iced::{Point, Size};
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::widget::canvas::{Canvas, Cache};
use iced::{executor, keyboard, Command, Subscription, Vector};
use iced::{Alignment, Application, Element, Settings};

use stroke::Stroke;
use canvas::*;


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


fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
