use crate::stroke::Stroke;

#[derive(Default, Debug, Clone)]
pub struct Scene {
    pub strokes: Vec<Stroke>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            strokes: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Stroke) {
        self.strokes.push(object);
    }
}
