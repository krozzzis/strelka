use iced::{
    widget::canvas::{self, Frame, Geometry, LineCap, LineDash, LineJoin, Style},
    Renderer,
};
use iced::{
    widget::canvas::{path, Path},
    Rectangle,
};

use crate::{stroke::Stroke, Scene};

pub struct SceneRenderer<'a> {
    scene: &'a Scene,
}

impl<'a> SceneRenderer<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self { scene }
    }

    pub fn construct_path(stroke: &Stroke) -> Path {
        let count = stroke.points.len();
        let mut builder = path::Builder::new();

        if let Some(start) = stroke.points.first() {
            builder.move_to(*start);
        }
        for i in 1..count {
            let point = stroke.points[i];
            builder.line_to(point);
        }

        builder.build()
    }

    pub fn draw(&self, renderer: &Renderer, bounds: Rectangle) -> Geometry {
        let mut frame = Frame::new(renderer, bounds.size());

        for stroke in &self.scene.strokes {
            let path = Self::construct_path(stroke);

            frame.stroke(
                &path,
                canvas::Stroke {
                    style: Style::Solid(stroke.color),
                    width: stroke.width,
                    line_cap: LineCap::Square,
                    line_join: LineJoin::Round,
                    line_dash: LineDash::default(),
                },
            );
        }

        frame.into_geometry()
    }
}
