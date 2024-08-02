use iced::widget::canvas::Frame;

use crate::{
    camera::Camera,
    scene::{Object, Rectangle, Spline},
};

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, _camera: &Camera) {
        for object in &self.objects {
            object.render(frame);
        }
    }

    pub fn add_spline(mut self, object: Spline) -> Self {
        self.objects.push(Box::new(object));
        self
    }

    pub fn add_rectangle(mut self, object: Rectangle) -> Self {
        self.objects.push(Box::new(object));
        self
    }
}
