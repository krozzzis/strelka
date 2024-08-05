use iced::{
    alignment::Vertical,
    border::Radius,
    widget::{column, component, container::Style, row, text, Component, Container, Toggler},
    Color, Element, Font, Length, Pixels,
};

use crate::plugin::{PluginId, PluginInfo, PluginStatus};

/// Plugin list entry widget
pub struct PluginEntry<'a> {
    pub info: &'a PluginInfo,
    pub status: PluginStatus,
}

/// Plugin list widget
pub struct PluginList<'a, Message> {
    pub plugins: Vec<PluginEntry<'a>>,
    pub change_status: Box<dyn Fn(PluginId, bool) -> Message + 'a>,
}

/// Create plugin list widget
#[allow(dead_code)]
pub fn plugin_list<'a, Message, F>(
    plugins: Vec<PluginEntry<'a>>,
    change_status: F,
) -> PluginList<'a, Message>
where
    F: 'a + Fn(PluginId, bool) -> Message,
{
    PluginList {
        plugins,
        change_status: Box::new(change_status),
    }
}

impl<'a, Message> Component<Message> for PluginList<'a, Message> {
    type State = ();

    type Event = Message;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let mut entries = Vec::new();
        for plugin in &self.plugins {
            let entry = Container::new(plugin_list_entry(plugin, |state| {
                (self.change_status)(plugin.info.id.clone(), state)
            }))
            .width(Length::Fill);
            entries.push(entry.into());
        }
        Container::new(column(entries).spacing(8.0).width(Length::Fill)).into()
    }
}

impl<'a, Message> From<PluginList<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(plugin_list: PluginList<'a, Message>) -> Self {
        component(plugin_list)
    }
}

/// Create plugin list entry widget
pub fn plugin_list_entry<'a, Message: 'a, F>(
    plugin: &'a PluginEntry,
    change_status: F,
) -> Element<'a, Message>
where
    F: 'a + Fn(bool) -> Message,
{
    let name_font = Font {
        weight: iced::font::Weight::Medium,
        ..Font::default()
    };
    let meta_font = Font {
        weight: iced::font::Weight::Normal,
        ..Font::default()
    };
    let desc_font = Font {
        weight: iced::font::Weight::Normal,
        ..Font::default()
    };
    let meta_font_size = Pixels::from(12.0);
    let desc_font_size = Pixels::from(14.0);
    let name_font_size = Pixels::from(16.0);

    let mut meta = Vec::new();
    if let Some(author) = &plugin.info.author {
        meta.push(
            text(format!("By {}", author))
                .font(meta_font)
                .size(meta_font_size)
                .into(),
        )
    }
    if let Some(version) = &plugin.info.version {
        meta.push(
            text(format!("v. {}", version))
                .font(meta_font)
                .size(meta_font_size)
                .into(),
        )
    }

    let name = text(plugin.info.name.clone())
        .font(name_font)
        .size(name_font_size);

    let mut tags: Vec<Element<_>> = Vec::new();
    tags.push(name.into());
    tags.push(row(meta).spacing(8.0).into());

    if let Some(description) = &plugin.info.description {
        let desc = text(description.clone())
            .font(desc_font)
            .size(desc_font_size);
        tags.push(desc.into());
    }

    let loaded = matches!(plugin.status, PluginStatus::Loaded);

    Container::new(
        row![
            column(tags).spacing(4.0).width(Length::Fill),
            Toggler::new(String::from(""), loaded, change_status)
        ]
        .spacing(16.0)
        .align_y(Vertical::Center),
    )
    .padding(12.0)
    .width(Length::Fill)
    .style(move |_theme| {
        let mut style: Style = Color::new(0.95, 0.95, 0.95, 1.0).into();
        style.border.radius = Radius::new(8.0);
        style
    })
    .into()
}
