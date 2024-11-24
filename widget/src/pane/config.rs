use core::{smol_str::SmolStr, value::Value};

use iced::{
    widget::{row, text, Space},
    Alignment, Element, Length, Padding,
};
use theming::Theme;

use crate::container::background2;

pub fn config_pane<'a>() -> Element<'a, (), Theme> {
    todo!();
    // let properties = ;
    // let entries: Vec<_> = properties
    //     .map(|(namespace, property, value)| {
    //         config_entry(namespace.clone(), property.clone(), value.clone())
    //     })
    //     .collect();
    // background(center(scrollable(Column::from_vec(entries).spacing(16.0)))).into()
}

pub fn config_entry<'a>(
    namespace: SmolStr,
    property: SmolStr,
    value: Value,
) -> Element<'a, (), Theme> {
    background2(row![
        text(format!("{}: ", namespace)),
        text(property.to_string()),
        Space::with_width(Length::Fill),
        config_value(&value)
    ])
    .width(Length::Fixed(700.0))
    .align_y(Alignment::Center)
    .padding(Padding::new(0.0).left(16.0).right(16.0))
    .height(Length::Fixed(36.0))
    .into()
}

pub fn config_value<'a>(value: &Value) -> Element<'a, (), Theme> {
    let value_view: Element<'a, (), Theme> = match value {
        Value::Integer(number) => text(number.to_string()).into(),
        Value::Float(number) => text(number.to_string()).into(),
        Value::Boolean(boolean) => if *boolean {
            text("True")
        } else {
            text("False")
        }
        .into(),
        Value::Color(color) => text(format!(
            "rgba({} {} {} {})",
            color.r, color.g, color.b, color.a
        ))
        .into(),
        Value::Path(_) => todo!(),
        Value::String(string) => text(format!("\"{}\"", string)).into(),
    };

    let value_type = match value {
        Value::Integer(_) => "Integer",
        Value::Float(_) => "Float",
        Value::Boolean(_) => "Boolean",
        Value::Color(_) => "Color",
        Value::Path(_) => "Path",
        Value::String(_) => "String",
    };

    row![text(value_type), text(":"), value_view]
        .spacing(2.0)
        .into()
}
