mod message;
mod screen;

use std::sync::Arc;

use iced::{Element, Task};
use strelka_core::Core;

use message::Message;
use screen::{BufferView, FileExplorer, Screen, ScreenMessage};

struct Strelka {
    core: Arc<Core>,
    screen: Screen,
}

impl Strelka {
    fn new() -> (Self, Task<Message>) {
        let core = Arc::new(Core::new());
        let mut tasks = Vec::new();

        let explorer = FileExplorer::new("./");
        let init = explorer
            .init(&core)
            .map(|e| Message::Screen(ScreenMessage::FileExplorer(e)));
        tasks.push(init);

        (
            Self {
                core,
                screen: Screen::FileExplorer(explorer),
            },
            Task::batch(tasks),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("Message: {message:?}");
        match message {
            Message::CoreCommand(cmd) => {
                let core = self.core.clone();
                Task::perform(async move { core.handle_command(cmd).await }, |_| {
                    Message::None
                })
            }
            Message::SetScreen(screen) => {
                self.screen = *screen;
                self.screen.init(&self.core)
            }
            Message::Screen(screen_event) => match screen_event {
                ScreenMessage::BufferView(e) => {
                    if let Screen::BufferView(state) = &mut self.screen {
                        state.update(&self.core, e)
                    } else {
                        Task::none()
                    }
                }
                ScreenMessage::FileExplorer(e) => {
                    if let Screen::FileExplorer(state) = &mut self.screen {
                        state.update(&self.core, e)
                    } else {
                        Task::none()
                    }
                }
            },
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.screen.view(&self.core)
    }
}

pub fn main() -> iced::Result {
    iced::application(Strelka::new, Strelka::update, Strelka::view).run()
}
