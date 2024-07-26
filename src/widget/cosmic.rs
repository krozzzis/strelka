use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use iced::{
    widget::{
        canvas::{Frame, Geometry, Program},
        Canvas,
    },
    Point, Rectangle, Renderer, Size, Theme,
};

use crate::AppMessage;

pub struct TextEditorCache {
    pub swash: SwashCache,
    pub font_system: FontSystem,
}

impl Default for TextEditorCache {
    fn default() -> Self {
        Self {
            swash: SwashCache::new(),
            font_system: FontSystem::new(),
        }
    }
}

pub struct Cosmic;

impl Program<AppMessage> for Cosmic {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<Geometry> {
        // let mut cache = self.cache.lock().unwrap();

        let mut font_system = FontSystem::new();
        let mut swash = SwashCache::new();

        // A SwashCache stores rasterized glyphs, create one per application
        // Text metrics indicate the font size and line height of a buffer
        let metrics = Metrics::new(48.0, 50.0);

        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = Buffer::new(&mut font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(&mut font_system);

        // Set a size for the text buffer, in pixels
        buffer.set_size(Some(bounds.width), Some(bounds.height));

        // Attributes indicate what font to choose
        let attrs = Attrs::new();

        // Add some text!
        buffer.set_text("Hello, Rust! 🦀\n", attrs, Shaping::Advanced);

        // Perform shaping as desired
        buffer.shape_until_scroll(true);

        // Inspect the output runs
        // for run in buffer.layout_runs() {
        //     for glyph in run.glyphs.iter() {
        //         println!("{:#?}", glyph);
        //     }
        // }

        // Create a default text color
        let text_color = Color::rgb(0x1, 0x1, 0x1);

        let mut frame = Frame::new(renderer, bounds.size());

        // Draw the buffer (for performance, instead use SwashCache directly)
        buffer.draw(&mut swash, text_color, |x, y, _w, _h, color| {
            let [r, g, b, a] = color.as_rgba();
            let color = iced::Color::from_rgba8(r, g, b, a as f32 / 256.0);
            frame.fill_rectangle(Point::new(x as f32, y as f32), Size::new(1.0, 1.0), color)
        });
        vec![frame.into_geometry()]
    }
}

pub fn cosmic_editor() -> Canvas<Cosmic, AppMessage> {
    Canvas::new(Cosmic {})
}
