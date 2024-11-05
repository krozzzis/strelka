use core::action::{Action, ActionWrapper, ThemeAction};

use log::{info, warn};
use theming::index::ThemeIndex;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ThemeActor {
    index: ThemeIndex,
    receiver: Receiver<ActionWrapper>,
    brocker_sender: Sender<ActionWrapper>,
}

impl ThemeActor {
    pub fn new(rx: Receiver<ActionWrapper>, brocker_tx: Sender<ActionWrapper>) -> Self {
        Self {
            index: ThemeIndex::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("ThemeActor. Started thread");
        while let Some(wrapper) = self.receiver.recv().await {
            info!("ThemeActor. Processing: {wrapper:?}");
            let action = if let Action::Theme(action) = wrapper.action() {
                action
            } else {
                warn!("ThemeActor. Dropping processing action because incorrect type");
                continue;
            };
            match action {
                ThemeAction::MakeIndex => {
                    if let Ok(index) = ThemeIndex::load_from_directory("./themes").await {
                        info!("Index: {index:?}");
                        self.index = index;
                    } else {
                        warn!("Can't make index from directory");
                    }
                }
                ThemeAction::SetTheme(id) => todo!(),
            }
        }
    }
}
