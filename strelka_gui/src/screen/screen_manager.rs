use std::collections::HashMap;
use std::sync::Arc;

use crate::message::Message;
use crate::screen::Screen;

use strelka_core::Core;

use iced::Element;
use iced::widget::pane_grid::{self, PaneGrid};

pub type ScreenId = u64;

pub struct ScreenManager {
    screens: HashMap<ScreenId, Screen>,
    pane_ids: HashMap<ScreenId, pane_grid::Pane>,
    pane_grid: pane_grid::State<ScreenId>,
}

impl ScreenManager {
    pub fn new() -> Self {
        let initial_screen_id = 0;
        let initial_screen: Screen = Screen::Blank;

        let mut screens = HashMap::new();
        screens.insert(initial_screen_id, initial_screen);

        let mut pane_ids = HashMap::new();
        let (pane_grid, pane_id) = pane_grid::State::new(initial_screen_id);
        pane_ids.insert(initial_screen_id, pane_id);

        Self {
            screens,
            pane_ids,
            pane_grid,
        }
    }

    pub fn view(&self, core: &Arc<Core>) -> Element<'_, Message> {
        let panes = PaneGrid::new(&self.pane_grid, |_pane_id, screen_id, _maximized| {
            pane_grid::Content::new(self.screens.get(screen_id).unwrap().view(*screen_id, core))
        });
        panes.width(iced::Fill).height(iced::Fill).into()
    }

    pub fn split_with(
        &mut self,
        screen_id: ScreenId,
        new_screen: impl Into<Screen>,
    ) -> Option<ScreenId> {
        if let Some(pane_id) = self.pane_ids.get(&screen_id) {
            let new_screen_id = self.screens.len() as u64;
            self.screens.insert(new_screen_id, new_screen.into());
            if let Some((new_pane_id, _)) =
                self.pane_grid
                    .split(pane_grid::Axis::Horizontal, *pane_id, new_screen_id)
            {
                self.pane_ids.insert(new_screen_id, new_pane_id);
                return Some(new_screen_id);
            }
        }

        None
    }
}
