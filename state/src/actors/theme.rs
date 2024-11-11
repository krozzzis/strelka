use action::{Action, ThemeAction};

use log::{info, warn};
use theming::{index::ThemeIndex, Theme};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ThemeActor {
    theme: Theme,
    index: ThemeIndex,
    receiver: Receiver<Action>,
    brocker_sender: Sender<Action>,
}

impl ThemeActor {
    pub fn new(rx: Receiver<Action>, brocker_tx: Sender<Action>) -> Self {
        Self {
            theme: Theme::default(),
            index: ThemeIndex::new(),
            receiver: rx,
            brocker_sender: brocker_tx,
        }
    }

    pub async fn run(&mut self) {
        info!("Started ThemeActor");
        while let Some(generic_action) = self.receiver.recv().await {
            info!("ThemeActor. Processing: {generic_action:?}");
            let action = if let Ok(x) = generic_action.content.downcast() {
                x
            } else {
                warn!("ThemeActor. Dropping processing action because incorrect type");
                continue;
            };
            match *action {
                ThemeAction::MakeIndex => {
                    if let Ok(index) = ThemeIndex::load_from_directory("./themes").await {
                        info!("Index: {index:?}");
                        self.index = index;
                    } else {
                        warn!("Can't make index from directory");
                    }
                }
                ThemeAction::SetTheme(id) => {
                    if let Some(dir_path) = self.index.get_path(&id) {
                        let mut path = dir_path.to_path_buf();
                        path.push("theme.toml");
                        let theme = Theme::from_file(&path).await;
                        if let Ok(theme) = theme {
                            self.theme = theme;
                            info!("Set theme {id}");
                        } else {
                            warn!("Can't load theme '{id}' from file '{path:?}'");
                        }
                    } else {
                        warn!("Theme '{id} not found");
                    }
                }
                ThemeAction::GetCurrentTheme(sender) => {
                    let _ = sender.send(self.theme.clone()).await;
                }
            }
        }
    }
}
