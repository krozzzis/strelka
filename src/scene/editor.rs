use crate::{
    scene::{CanvasProgram, CanvasState, Scene},
    stroke::Stroke,
};

use iced::widget::{component, Component};
use iced::Point;
use iced::{widget::Canvas, Element, Length};

#[derive(Debug, Clone)]
pub enum EditorMessage {
    AddPointToPending(Point),
    AddPendingToScene,
}

pub struct SceneEditor<'a, Message> {
    scene: &'a Scene,
    state: CanvasState,
    add_object: Option<Box<dyn Fn(Stroke) -> Message>>,
}

impl<'a, Message> SceneEditor<'a, Message> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            scene,
            state: CanvasState::default(),
            add_object: None,
        }
    }

    pub fn on_added_object(mut self, func: impl Fn(Stroke) -> Message + 'static) -> Self {
        self.add_object = Some(Box::new(func));
        self
    }
}

impl<'a, Message> Component<Message> for SceneEditor<'a, Message> {
    type State = ();
    type Event = EditorMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            EditorMessage::AddPointToPending(point) => {
                self.state.pending_stroke.points.push(point);
            }

            EditorMessage::AddPendingToScene => {
                let stroke = self.state.pending_stroke.clone();
                self.state.pending_stroke.points.clear();

                if let Some(func) = &self.add_object {
                    return Some(func(stroke));
                }
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let canvas = Canvas::new(CanvasProgram::new(&self.state, self.scene))
            .width(Length::Fill)
            .height(Length::Fill);

        canvas.into()
    }
}

impl<'a, Message> From<SceneEditor<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(scene_editor: SceneEditor<'a, Message>) -> Self {
        component(scene_editor)
    }
}

pub fn scene_editor<Message>(scene: &Scene) -> SceneEditor<Message> {
    SceneEditor::new(scene)
}
