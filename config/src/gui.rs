use strelka_core::ThemeId;

use theming::Theme;

pub enum InterfaceMode {
    Simplified,
    Full,
}

pub struct GuiConfig {
    pub theme_id: ThemeId,
    pub theme: Theme,
    pub interface_mode: InterfaceMode,
    pub scale_factor: f64,
}
