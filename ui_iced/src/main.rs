use iced::{
    executor,
    widget::{button, container, row, text},
    Application, Command, Element, Error, Renderer, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    Notes::run(Settings::default())
}

struct Notes;

#[derive(Clone, Debug)]
enum Message {
    CreateNote,
    NoteCreated,
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

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CreateNote => Command::none(),
            Message::NoteCreated => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let create_note_btn = button("New").on_press(Message::CreateNote);
        // create_note_btn

        // text("Hello").into()
        container(create_note_btn).into()
    }

    // fn view(&self) -> Element<'_, Message> {
    //     let create_note_btn = button("New").on_press(Message::CreateNote);

    //     // container(column!([create_note_btn]))
    //     // let side_menu =
    //     // widget::text("Hello").into()
    //     // create_note_btn.into()
    //     // let mut widgets = vec![[]];
    //     // // widgets.push(text("Hellot"));
    //     // let create_note_btn = iced::widget::button(content)
    //     // container(row!(widgets)).into()
    // }
}

fn create_new_note() -> Result<(), Error> {
    print!("create_new_note");
    todo!();
}
