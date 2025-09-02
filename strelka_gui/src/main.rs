mod document_view;

use iced::{Element, Task};
use smol_str::SmolStr;
use strelka_core::{Core};
use strelka_core::Document;
use crate::document_view::DocumentView;
use iced::widget::{center, column};


struct Strelka {
    core: Core,
    document_view: Vec<DocumentView>,
}

impl Strelka {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                core: Core::new(),
                document_view: vec![
                    DocumentView {
                        id: SmolStr::from("aboba"),
                        title: SmolStr::from("title1"),
                        content_preview: SmolStr::from("content1"),
                    },
                    DocumentView {
                        id: SmolStr::from("aboba"),
                        title: SmolStr::from("title2"),
                        content_preview: SmolStr::from("content2"),
                    }
                ],
            },
            Task::none()
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column(self.document_view.iter().map(|doc| doc.render())).spacing(24.0).into()
    }

}

#[derive(Debug, Clone)]
enum Message {
    None,
}



pub fn main() -> iced::Result {
    iced::application(Strelka::new, Strelka::update, Strelka::view)
        .run()
}