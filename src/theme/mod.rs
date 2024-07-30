use iced::Color;

#[derive(Clone)]
pub struct Theme {
    pub background: Color,
    pub background2: Color,
    pub text: Color,
    pub subtext: Color,
    pub selected: Color,
    pub primary: Color,
    pub border_color: Color,
    pub element_radius: f32,
    pub element_padding: f32,
}

impl Theme {
    /// Dark theme from material ui
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb8(18, 18, 18),
            background2: Color::from_rgb8(36, 36, 36),
            text: Color::from_rgb8(255, 255, 255),
            subtext: Color::from_rgb8(178, 178, 178),
            selected: Color::from_rgb8(40, 40, 40),
            primary: Color::from_rgb8(245, 224, 220),
            border_color: Color::from_rgb8(40, 40, 40),
            element_radius: 4.0,
            element_padding: 4.0,
        }
    }
}

impl Default for Theme {
    /// Light theme from Catppuccin Latte
    fn default() -> Self {
        Self {
            background: Color::from_rgb8(239, 241, 245),
            background2: Color::from_rgb8(204, 208, 218),
            text: Color::from_rgb8(76, 79, 105),
            subtext: Color::from_rgb8(92, 95, 119),
            selected: Color::from_rgb8(156, 160, 176),
            primary: Color::from_rgb8(220, 138, 120),
            border_color: Color::from_rgb8(124, 127, 147),
            element_radius: 4.0,
            element_padding: 4.0,
        }
    }
}