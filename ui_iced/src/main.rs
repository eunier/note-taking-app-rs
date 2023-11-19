use iced::{
    executor,
    widget::{container, row, text},
    Application, Command, Element, Sandbox, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    Notes::run(Settings::default())
}

struct Notes;

#[derive(Debug)]
enum Message {
    NewNote,
}

impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // let side_menu =
        text("Hello").into()
        // let mut widgets = vec![[]];
        // // widgets.push(text("Hellot"));
        // let new_note_btn = iced::widget::button(content)
        // container(row!(widgets)).into()
    }
}
