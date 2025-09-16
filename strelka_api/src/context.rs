use std::sync::Arc;

use crate::core::CoreAPI;
use crate::window::GuiService;

#[derive(Clone)]
pub struct ActionContext {
    pub core: Arc<dyn CoreAPI>,
    pub gui: Arc<dyn GuiService>,
}
