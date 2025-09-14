mod header_bar;
mod message;
mod screen;

use std::sync::Arc;

use iced::widget::column;
use iced::{Element, Task};

use strelka_api::core::CoreAPI;
use strelka_api::message::{PluginMessage, WindowMessage};
use strelka_core::Core;
use strelka_plugin::ActionRegistry;

use crate::header_bar::header_bar;
use message::Message;
use screen::{BufferView, FileExplorer, Screen, ScreenMessage};

struct Strelka {
    core: Arc<Core>,
    screen: Screen,
    action_registry: Arc<ActionRegistry>,
    window_id: Option<iced::window::Id>,
}

impl Strelka {
    fn new() -> (Self, Task<Message>) {
        let core = Arc::new(Core::new());
        let action_registry = Arc::new(ActionRegistry::new(core.clone()));

        action_registry.register("print_hello", async |_core| {
            println!("Hello");
            PluginMessage::None
        });

        let mut tasks = Vec::new();

        let obtain_id = iced::window::latest()
            .map(|id| Message::SetWindowId(id.expect("Cannot get window id")));
        tasks.push(obtain_id);

        let explorer = FileExplorer::new("./");
        let init = explorer
            .init(&core)
            .map(|e| Message::Screen(ScreenMessage::FileExplorer(e)));
        tasks.push(init);

        (
            Self {
                core,
                action_registry,
                window_id: None,
                screen: Screen::FileExplorer(explorer),
            },
            Task::batch(tasks),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("Message: {message:?}");
        match message {
            Message::CoreMessage(cmd) => {
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
            Message::Window(msg) => {
                if let Some(window_id) = self.window_id {
                    match msg {
                        WindowMessage::Close => iced::window::close(window_id),
                        WindowMessage::ToggleMaximize => iced::window::toggle_maximize(window_id),
                        WindowMessage::Collapse => iced::window::minimize(window_id, true),
                        WindowMessage::DragStart => iced::window::drag(window_id),
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::SetWindowId(id) => {
                self.window_id = Some(id);
                Task::none()
            }
            Message::Action(action) => {
                let registry = self.action_registry.clone();
                Task::perform(async move { registry.execute(action).await }, |_| {
                    Message::None
                })
            }
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            header_bar().map(Message::Window),
            self.screen.view(&self.core),
        ]
        .into()
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }
}

pub fn main() -> iced::Result {
    iced::application(Strelka::new, Strelka::update, Strelka::view)
        .title(Strelka::title)
        .run()
}
