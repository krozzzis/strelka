mod application;
mod message;
pub mod widget;

use crate::application::Strelka;

pub fn main() -> iced::Result {
    iced::daemon(Strelka::new, Strelka::update, Strelka::view)
        .title(Strelka::title)
        .subscription(Strelka::subscription)
        .run()
}
