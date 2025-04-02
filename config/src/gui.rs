use strelka_core::ThemeId;

pub enum InterfaceMode {
    Simplified,
    Full,
}

pub struct GuiConfig {
    pub theme_id: ThemeId,
    pub interface_mode: InterfaceMode,
    pub scale_factor: f64,
}
