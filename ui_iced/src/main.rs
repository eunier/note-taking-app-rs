use iced::{
    executor,
    widget::{button, container, row, text},
    Application, Command, Element, Renderer, Settings, Theme,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Notes::run(Settings::default())
    // match db::connect().await {
    //     Ok(db) => Ok(()),
    //     Err(e) => Err(e),
    // }
    let db = db::connect().await?;
    // let mut settings = Settings::default();
    // settings.flags = Flags { db };
    Ok(Notes::run(Settings {
        // flags: Flags { db },
        flags: 1.0,
        ..Default::default()
    })?)
    // Ok(())
}

struct Flags {
    // db: Arc<Mutex<db::DB>>,
    // db: Surreal<Db>,
    db: f32,
}

struct Notes {
    // db: Surreal<Db>,
    db: f32,
}

#[derive(Clone, Debug)]
enum Message {
    CreateNoteClick,
}

impl Application for Notes {
    type Executor = executor::Default;
    type Flags = f32;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { db: flags }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CreateNoteClick => {
                dbg!("Click");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let create_note_btn = button("New").on_press(Message::CreateNoteClick);
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
