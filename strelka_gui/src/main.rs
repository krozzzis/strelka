mod screen;

use crossbeam_channel::{unbounded, Sender, Receiver};

use std::path::PathBuf;
use std::sync::Arc;

use iced::{Element, Task};
use iced::widget::{container, row, text};
use strelka_core::{Core, CoreCommand};
use iced::widget::column;

use screen::{Screen, BufferView};

struct Strelka<'a> {
    core: Arc<Core>,
    screen: Box<dyn Screen<'a, Message>>,
}

impl Strelka<'_> {
    fn new() -> (Self, Task<Message>) {
        let (cmd_tx, cmd_rx) = unbounded();
        let (event_tx, event_rx) = unbounded();

        let core = Arc::new(Core::new(cmd_rx, event_tx.clone()));

        let open_tasks = std::fs::read_dir(".")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| {
                let path = entry.path();
                let cmd = CoreCommand {
                    action: strelka_core::CoreAction::OpenFile(path),
                };
                Message::CoreCommand(cmd)
            })
            .collect::<Vec<_>>();

        let task = Task::batch(open_tasks.into_iter().map(Task::done));

        (
            Self {
                core,
                screen: Box::new(BufferView::new(1) )
            },
            
            task
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CoreCommand(cmd) => {
                let core = self.core.clone();
                Task::perform(async move {
                    core.handle_command(cmd).await
                }, |_| Message::None)
            },
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.screen.view(self.core.clone()).map(|_| Message::None)
    }

}

#[derive(Debug, Clone)]
enum Message {
    CoreCommand(CoreCommand),
    None,
}

pub fn main() -> iced::Result {
    iced::application(Strelka::new, Strelka::update, Strelka::view)
        .run()
}