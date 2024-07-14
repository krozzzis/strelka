use iced::{
    widget::canvas::{self, path, Frame, LineCap, LineDash, LineJoin, Path, Style},
    Color, Point, Size,
};

pub trait Object {
    fn render(&self, frame: &mut Frame);
    fn translate(&mut self, offset: Point);
}

pub struct Rectangle {
    pub position: Point,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub width: f32,
}

impl Object for Rectangle {
    fn render(&self, frame: &mut Frame) {
        let path = Path::rectangle(self.position, Size::new(self.w, self.h));

        frame.stroke(
            &path,
            canvas::Stroke {
                style: Style::Solid(self.color),
                width: self.width,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                line_dash: LineDash::default(),
            },
        );
    }

    fn translate(&mut self, offset: Point) {
        self.position.x += offset.x;
        self.position.y += offset.y;
    }
}

#[derive(Debug, Clone)]
pub struct Spline {
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f32,
}

impl Object for Spline {
    fn render(&self, frame: &mut Frame) {
        let count = self.points.len();
        let mut builder = path::Builder::new();

        if let Some(start) = self.points.first() {
            builder.move_to(Point::new(start.x, start.y));
        }
        for i in 1..count {
            let point = self.points[i];
            let point = Point::new(point.x, point.y);
            builder.line_to(point);
        }

        let path = builder.build();
        frame.stroke(
            &path,
            canvas::Stroke {
                style: Style::Solid(self.color),
                width: self.width,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                line_dash: LineDash::default(),
            },
        );
    }

    fn translate(&mut self, offset: Point) {
        for point in self.points.iter_mut() {
            point.x += offset.x;
            point.y += offset.y;
        }
    }
}

impl Spline {
    // pub fn calculate_bb(&mut self) {
    //     let mut min_x = self.points[0].x;
    //     let mut min_y = self.points[0].y;
    //     let mut max_x = self.points[0].x;
    //     let mut max_y = self.points[0].y;
    //
    //     for pos in &self.points[1..] {
    //         max_x = max_x.max(pos.x);
    //         max_y = max_y.max(pos.y);
    //         min_x = min_x.min(pos.x);
    //         min_y = min_y.min(pos.y);
    //     }
    //
    //     self.bb_start = Point::new(min_x, min_y);
    //     self.bb_size = Size::new(max_x - min_x, max_y - min_y);
    // }
}

impl Default for Spline {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: Color::BLACK,
            width: 4.0,
        }
    }
}
