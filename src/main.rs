use iced::{widget::text, Element, Sandbox, Settings};

fn main() -> Result<(), iced::Error> {
    Notes::run(Settings::default())
}

struct Notes;

#[derive(Debug)]
enum Message {}

impl Sandbox for Notes {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Hello").into()
    }
}
