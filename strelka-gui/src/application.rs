use iced::{Element, Subscription, Task, widget::text};

use crate::message::{self, Message};
use tracing::{debug, info};

pub struct Strelka {
    window_id: iced::window::Id,
}

impl Strelka {
    pub fn new() -> (Self, Task<Message>) {
        let (main_window_id, open_main_window) = iced::window::open(iced::window::Settings {
            exit_on_close_request: false,
            ..iced::window::Settings::default()
        });

        let tasks = vec![
            open_main_window
                .map(|_| Message::Window(message::WindowMessage::InitializedMainWindow)),
        ];

        (
            Self {
                window_id: main_window_id,
            },
            Task::batch(tasks),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(msg) => match msg {
                message::WindowMessage::InitializedMainWindow => {
                    debug!("Main window initialized")
                }
                message::WindowMessage::Close(id) => {
                    let mut close_task = iced::window::close(id);
                    // Close an entire application if we trying to close main window
                    if id == self.window_id {
                        close_task = close_task.chain(self.exit());
                    }
                    return close_task;
                }
            },
            Message::None => {}
        }

        Task::none()
    }

    pub fn view(&self, _window_id: iced::window::Id) -> Element<'_, Message> {
        text("Nothing here").into()
    }

    pub fn title(&self, _window_id: iced::window::Id) -> String {
        String::from("Strelka")
    }

    fn exit(&mut self) -> Task<Message> {
        info!("Closing application gracefully");

        iced::exit()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let tasks = vec![
            iced::window::close_requests()
                .map(|id| Message::Window(message::WindowMessage::Close(id))),
        ];
        Subscription::batch(tasks)
    }
}
