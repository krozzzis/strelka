use std::sync::Arc;

use iced::{advanced::graphics::core::SmolStr, keyboard::Modifiers};

use crate::plugin::{self, Hotkey, Plugin, PluginAction, PluginMessage};

pub struct ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn update(&mut self, message: Arc<PluginMessage>) -> Option<PluginAction> {
        if message.kind == "say" {
            println!("{}", message.payload);
            Some(PluginAction::SendNotification(Arc::new(format!(
                "Message from plugin: {}",
                message.payload
            ))))
        } else {
            None
        }
    }

    fn load(&mut self) -> Option<PluginAction> {
        println!("Example plugin loaded");
        Some(PluginAction::RegisterHotkey(
            Hotkey {
                key: iced::keyboard::Key::Character(SmolStr::new_inline("f")),
                modifiers: Modifiers::CTRL,
            },
            Arc::new(PluginMessage {
                kind: "say".to_owned(),
                payload: "Hotkey!!!".to_owned(),
            }),
        ))
    }

    fn unload(&mut self) -> Option<PluginAction> {
        println!("Example plugin unloaded");
        None
    }
}
