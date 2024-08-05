use crate::theming::styles::button::Button;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub button: Button,
}
