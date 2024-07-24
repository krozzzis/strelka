use iced::{
    font::{Family, Stretch, Style, Weight},
    widget::button,
    Font, Theme,
};

pub const INTER_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../content/Inter-Regular.ttf");

pub const INTER_REGULAR_FONT: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

// pub fn BUTTON_STYLE(theme: &Theme, status: button::Status) -> button::Style {
//     button::Style {
//         background: Color::new(
//         text_color: todo!(),
//         border: todo!(),
//         shadow: todo!(),
//     }
// }
