#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HotKey {
    pub modifiers: Modifiers,
    pub key: char,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Modifiers {
    #[default]
    None,
    Ctrl,
    Alt,
    CtrlAlt,
}
