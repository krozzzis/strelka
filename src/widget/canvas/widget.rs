use std::sync::Arc;

use crate::scene::Scene;
use crate::{plugin, widget::canvas::CanvasProgram};

use iced::widget::Canvas as IcedCanvas;
use iced::widget::{canvas::Cache, component, Component};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    SendPluginAction {
        plugin: String,
        action: Arc<plugin::Action>,
    },
}

pub struct Canvas<'a, Message> {
    scene: &'a Scene,
    cache: Cache,
    plugin_take_action: Option<Box<dyn Fn(String, Arc<plugin::Action>) -> Message>>,
}

impl<'a, Message> Canvas<'a, Message> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            scene,
            cache: Cache::default(),
            plugin_take_action: None,
        }
    }

    pub fn on_plugin_action(
        mut self,
        func: impl Fn(String, Arc<plugin::Action>) -> Message + 'static,
    ) -> Self {
        self.plugin_take_action = Some(Box::new(func));
        self
    }
}

impl<'a, Message> Component<Message> for Canvas<'a, Message> {
    type State = ();
    type Event = CanvasMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            CanvasMessage::SendPluginAction {
                plugin,
                action: message,
            } => {
                if let Some(func) = &self.plugin_take_action {
                    return Some(func(plugin, message));
                }
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let canvas = IcedCanvas::new(CanvasProgram {
            cache: &self.cache,
            scene: self.scene,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        canvas.into()
    }
}

impl<'a, Message> From<Canvas<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(scene_editor: Canvas<'a, Message>) -> Self {
        component(scene_editor)
    }
}

pub fn canvas<Message>(scene: &Scene) -> Canvas<Message> {
    Canvas::new(scene)
}
