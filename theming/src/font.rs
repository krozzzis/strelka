use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Font {
    pub family: Cow<'static, str>,
    pub weight: Weight,
    pub style: Style,
    pub size: f32,
}

impl Font {
    pub const SANS_SERIF: Font = Font {
        family: Cow::Borrowed("Sans Serif"),
        weight: Weight::Normal,
        style: Style::Normal,
        size: 20.0,
    };
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Sans Serif"),
            weight: Weight::Normal,
            style: Style::Normal,
            size: 20.0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum Weight {
    Light,
    #[default]
    Normal,
    SemiBold,
    Bold,
}

#[derive(Debug, Default, Clone)]
pub enum Style {
    #[default]
    Normal,
    Italic,
}
