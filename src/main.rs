use iced::{widget::text, Element, Sandbox, Settings};

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}

struct App;

#[derive(Debug)]
enum Message {}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Hello").into()
    }
}
