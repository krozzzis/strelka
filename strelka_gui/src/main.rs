mod header_bar;
mod message;
mod screen;

use std::sync::Arc;

use iced::Subscription;
use iced::futures::{Stream, sink::SinkExt};
use iced::stream;
use iced::widget::column;
use iced::{Element, Task};

use tokio::sync::mpsc::{Sender, channel};

use strelka_api::context::ActionContext;
use strelka_api::core::CoreAPI;
use strelka_api::message::{PluginMessage, WindowMessage};
use strelka_core::Core;
use strelka_core::MessageBasedGuiService;
use strelka_plugin::ActionRegistry;

use crate::header_bar::header_bar;
use crate::screen::{BufferView, FileExplorer, Screen, ScreenMessage};
use message::Message;

struct Strelka {
    core: Arc<Core>,
    screen: Screen,
    action_registry: Option<Arc<ActionRegistry>>,
    window_id: Option<iced::window::Id>,
}

impl Strelka {
    fn new() -> (Self, Task<Message>) {
        let core = Arc::new(Core::new());
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
                action_registry: None,
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
                ScreenMessage::BufferList(e) => {
                    if let Screen::BufferList(state) = &mut self.screen {
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
            Message::Action(action, arg) => {
                if let Some(registry) = self.action_registry.clone() {
                    Task::perform(async move { registry.execute(action, arg).await }, |_| {
                        Message::None
                    })
                } else {
                    Task::none()
                }
            }
            Message::GUIChannelEstablised(sender) => {
                self.init_action_registry(sender);
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![header_bar(), self.screen.view(&self.core),].into()
    }

    fn title(&self) -> String {
        String::from("Strelka")
    }

    fn init_action_registry(&mut self, sender: Sender<WindowMessage>) {
        let gui_service = MessageBasedGuiService::new(sender);

        let context = ActionContext {
            core: self.core.clone(),
            gui: Arc::new(gui_service),
        };

        let action_registry = Arc::new(ActionRegistry::new(context));

        action_registry.register("close_window", async |ctx, _arg| {
            ctx.gui.send_window_message(WindowMessage::Close).await.ok();
            PluginMessage::None
        });

        action_registry.register("collapse_window", async |ctx, _arg| {
            ctx.gui
                .send_window_message(WindowMessage::Collapse)
                .await
                .ok();
            PluginMessage::None
        });

        action_registry.register("toggle_maximize_window", async |ctx, _arg| {
            ctx.gui
                .send_window_message(WindowMessage::ToggleMaximize)
                .await
                .ok();
            PluginMessage::None
        });

        action_registry.register("start_drag_window", async |ctx, _arg| {
            ctx.gui
                .send_window_message(WindowMessage::DragStart)
                .await
                .ok();
            PluginMessage::None
        });

        self.action_registry = Some(action_registry);
    }

    fn window_message_worker() -> impl Stream<Item = Message> {
        stream::channel(
            100,
            |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                let (sender, mut receiver) = channel::<WindowMessage>(100);

                output
                    .send(Message::GUIChannelEstablised(sender))
                    .await
                    .ok();

                while let Some(msg) = receiver.recv().await {
                    output.send(Message::Window(msg)).await.ok();
                }
            },
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(Self::window_message_worker)
    }
}

pub fn main() -> iced::Result {
    iced::application(Strelka::new, Strelka::update, Strelka::view)
        .title(Strelka::title)
        .subscription(Strelka::subscription)
        .run()
}
