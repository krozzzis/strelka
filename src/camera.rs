use iced::Point;

#[derive(Default, Debug, Clone)]
pub struct Camera {
    pub position: Point,
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Point::ORIGIN,
            scale: 1.0,
        }
    }

    pub fn translate(&mut self, offset: Point) {
        self.position.x += offset.x;
        self.position.y += offset.y;
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }
}
