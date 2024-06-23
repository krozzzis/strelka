use iced::{Color, Point, Size};

#[derive(Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f32,
    pub bb_start: Point,
    pub bb_size: Size,
}

impl Stroke {
    pub fn calculate_bb(&mut self) {
        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;

        for pos in &self.points[1..] {
            max_x = max_x.max(pos.x);
            max_y = max_y.max(pos.y);
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
        }

        self.bb_start = Point::new(min_x, min_y);
        self.bb_size = Size::new(max_x - min_x, max_y - min_y);
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: Color::BLACK,
            width: 4.0,
            bb_start: Point::ORIGIN,
            bb_size: Size::ZERO,
        }
    }
}
