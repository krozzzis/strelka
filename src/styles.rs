use iced::{
    font::{Family, Stretch, Style, Weight},
    Font,
};

pub const INTER_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../content/Inter-Regular.ttf");

pub const INTER_REGULAR_FONT: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
