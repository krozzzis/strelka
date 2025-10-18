use std::sync::Arc;

use iced::Subscription;
use iced::futures::{Stream, sink::SinkExt};
use iced::stream;
use iced::{Element, Task};

use tokio::sync::mpsc::{Sender, channel};

use strelka_api::context::ActionContext;
use strelka_api::core::CoreAPI;
use strelka_api::message::{PluginMessage, WindowMessage};
use strelka_core::Core;
use strelka_core::MessageBasedGuiService;
use strelka_plugin::ActionRegistry;

use crate::message::Message;
use crate::widget::file_manager;

pub struct Strelka {
    pub core: Arc<Core>,
    pub action_registry: Option<Arc<ActionRegistry>>,
    file_manager: file_manager::State,
    context: Option<ActionContext>,
    window_id: iced::window::Id,
}

impl Strelka {
    pub fn new() -> (Self, Task<Message>) {
        let core = Arc::new(Core::new());

        let (main_window_id, open_main_window) = iced::window::open(iced::window::Settings {
            exit_on_close_request: false,
            ..iced::window::Settings::default()
        });

        let (file_manager_state, file_manager_init) = file_manager::State::new("./");

        let tasks = vec![
            open_main_window.then(|_| Task::none()),
            file_manager_init.map(Message::FileManager),
        ];

        (
            Self {
                core,
                file_manager: file_manager_state,
                action_registry: None,
                window_id: main_window_id,
                context: None,
            },
            Task::batch(tasks),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("Message: {message:?}");
        match message {
            Message::CoreMessage(cmd) => {
                let core = self.core.clone();
                return Task::perform(async move { core.handle_command(cmd).await }, |_| {
                    Message::None
                });
            }
            Message::Window(msg) => match msg {
                WindowMessage::Close => return iced::window::close(self.window_id),
                WindowMessage::ToggleMaximize => {
                    return iced::window::toggle_maximize(self.window_id);
                }
                WindowMessage::Collapse => return iced::window::minimize(self.window_id, true),
                WindowMessage::DragStart => return iced::window::drag(self.window_id),
                WindowMessage::DragEnd => {}
                WindowMessage::ExitRequest => return iced::exit(),
                WindowMessage::Exit => return iced::exit(),
            },
            Message::Action(action, arg) => {
                if let Some(registry) = self.action_registry.clone() {
                    return Task::perform(
                        async move { registry.execute(action, arg).await },
                        |_| Message::None,
                    );
                }
            }
            Message::FileManager(message) => match message {
                file_manager::Message::Action(action) => {
                    return self
                        .file_manager
                        .take_action(action)
                        .map(Message::FileManager);
                }
                file_manager::Message::Open(path) => println!("Open {}", path.display()),
            },
            Message::GUIChannelEstablised(sender) => {
                self.init_action_registry(sender);
            }
            Message::None => {}
        }

        Task::none()
    }

    pub fn view(&self, _window_id: iced::window::Id) -> Element<'_, Message> {
        self.file_manager.view().map(Message::FileManager)
    }

    pub fn title(&self, _window_id: iced::window::Id) -> String {
        String::from("Strelka")
    }

    pub fn get_context(&self) -> Option<ActionContext> {
        self.context.clone()
    }

    fn init_action_registry(&mut self, sender: Sender<WindowMessage>) {
        let gui_service = MessageBasedGuiService::new(sender);

        let context = ActionContext {
            core: self.core.clone(),
            gui: Arc::new(gui_service),
        };

        self.context = Some(context.clone());

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

    pub fn subscription(&self) -> Subscription<Message> {
        let main_window_id = self.window_id;
        let tasks = vec![
            Subscription::run(Self::window_message_worker),
            iced::window::close_requests()
                .with(main_window_id)
                .map(|(main_window_id, id)| {
                    if id == main_window_id {
                        Message::Window(WindowMessage::ExitRequest)
                    } else {
                        Message::None
                    }
                }),
        ];
        Subscription::batch(tasks)
    }
}
