use db::{create_note, Record};
use iced::{
    executor,
    widget::{button, container},
    Application, Command, Element, Settings, Theme,
};

#[derive(Clone, Debug)]
enum Message {
    CreateDbNoteClicked,
    DbNoteCreated(Result<Vec<Record>, db::Error>),
}

struct Notes;

impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CreateDbNoteClicked => Command::perform(create_note(), Message::DbNoteCreated),
            Message::DbNoteCreated(_) => {
                dbg!("created");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let create_note_btn = button("New Database Note").on_press(Message::CreateDbNoteClicked);

        container(create_note_btn).into()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notes::run(Settings::default())?;
    // Ok(Notes::run(Arc::new(db), Settings::default())?)
    Ok(())
}

/* #region original2 */
// use std::sync::Arc;

// use db::{
//     create_note,
//     surrealdb::{self, engine::local::Db, Surreal},
//     Record,
// };
// use iced::{
//     executor,
//     widget::{button, container, row, text},
//     Application, Command, Element, Renderer, Settings, Theme,
// };

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Notes::run(Settings::default())
//     // match db::connect().await {
//     //     Ok(db) => Ok(()),
//     //     Err(e) => Err(e),
//     // }
//     // let db = db::connect().await?;
//     // let mut settings = Settings::default();
//     // settings.flags = Flags { db };
//     Ok(Notes::run(Settings {
//         // flags: Flags { db },
//         // flags: Flags { db: Some(db) },
//         ..Default::default() // TODO remove ..
//     })?)
//     // Ok(())
// }

// // #[derive(Default)]
// // struct Flags {
// //     // db: Arc<Mutex<db::DB>>,
// //     db: Option<Arc<Surreal<Db>>>, // TODO remove option
// //                                   // db: f32,
// // }

// struct Notes {
//     // db: Option<Arc<Surreal<Db>>>,
//     // db: f32,
// }

// #[derive(Clone, Debug)]
// enum Message {
//     CreateNoteClick,
//     NoteCreated(Result<Vec<Record>, db::Error>),
// }

// impl Application for Notes {
//     type Executor = executor::Default;
//     type Flags = ();
//     type Message = Message;
//     type Theme = Theme;

//     fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
//         (Self {}, Command::none())
//     }

//     fn title(&self) -> String {
//         String::from("Notes")
//     }

//     fn update(&mut self, message: Self::Message) -> Command<Message> {
//         match message {
//             Message::CreateNoteClick => {
//                 Command::perform(create_note(), Message::NoteCreated)
//             }
//             Message::NoteCreated(_) => {
//                 dbg!("created");
//                 Command::none()
//             }
//         }

//         // match &self.db {
//         //     Some(db) => match message {
//         //         Message::CreateNoteClick => {
//         //             // let create_note_cmd =  move  ||{
//         //             //     create_note(db)
//         //             // };

//         //             Command::perform(create_note(db), Message::NoteCreated)
//         //         }
//         //         Message::NoteCreated(_) => {
//         //             dbg!("created");
//         //             Command::none()
//         //         }
//         //     },
//         //     None => Command::none(),
//         // }

//         // match &self.db {
//         //     Some(db) => match message {
//         //         Message::CreateNoteClick => {
//         //             // let create_note_cmd =  move  ||{
//         //             //     create_note(db)
//         //             // };

//         //             Command::perform(create_note(db), Message::NoteCreated)
//         //         }
//         //         Message::NoteCreated(_) => {
//         //             dbg!("created");
//         //             Command::none()
//         //         }
//         //     },
//         //     None => Command::none(),
//         // }
//     }

//     fn view(&self) -> Element<'_, Message> {
//         let create_note_btn = button("New").on_press(Message::CreateNoteClick);
//         // create_note_btn

//         // text("Hello").into()
//         container(create_note_btn).into()
//     }

//     // fn view(&self) -> Element<'_, Message> {
//     //     let create_note_btn = button("New").on_press(Message::CreateNote);

//     //     // container(column!([create_note_btn]))
//     //     // let side_menu =
//     //     // widget::text("Hello").into()
//     //     // create_note_btn.into()
//     //     // let mut widgets = vec![[]];
//     //     // // widgets.push(text("Hellot"));
//     //     // let create_note_btn = iced::widget::button(content)
//     //     // container(row!(widgets)).into()
//     // }
// }
/* #region */

/* #region original */
// use db::{
//     create_note,
//     surrealdb::{self, engine::local::Db, Surreal},
//     Record,
// };
// use iced::{
//     executor,
//     widget::{button, container, row, text},
//     Application, Command, Element, Renderer, Settings, Theme,
// };

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Notes::run(Settings::default())
//     // match db::connect().await {
//     //     Ok(db) => Ok(()),
//     //     Err(e) => Err(e),
//     // }
//     let db = db::connect().await?;
//     // let mut settings = Settings::default();
//     // settings.flags = Flags { db };
//     Ok(Notes::run(Settings {
//         // flags: Flags { db },
//         flags: Flags { db: Some(db) },
//         ..Default::default()
//     })?)
//     // Ok(())
// }

// #[derive(Default)]
// struct Flags {
//     // db: Arc<Mutex<db::DB>>,
//     db: Option<Surreal<Db>>,
//     // db: f32,
// }

// struct Notes {
//     db: Option<Surreal<Db>>,
//     // db: f32,
// }

// #[derive(Clone, Debug)]
// enum Message {
//     CreateNoteClick,
//     NoteCreated(Result<Vec<Record>, db::Error>),
// }

// impl Application for Notes {
//     type Executor = executor::Default;
//     type Flags = Flags;
//     type Message = Message;
//     type Theme = Theme;

//     fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
//         (Self { db: flags.db }, Command::none())
//     }

//     fn title(&self) -> String {
//         String::from("Notes")
//     }

//     fn update(&mut self, message: Self::Message) -> Command<Message> {
//         match self.db {
//             Some(db) => match message {
//                 Message::CreateNoteClick => {
//                     Command::perform(create_note(&db), Message::NoteCreated)
//                 }
//                 Message::NoteCreated(_) => {
//                     dbg!("created");
//                     Command::none()
//                 }
//             },
//             None => Command::none(),
//         }
//     }

//     fn view(&self) -> Element<'_, Message> {
//         let create_note_btn = button("New").on_press(Message::CreateNoteClick);
//         // create_note_btn

//         // text("Hello").into()
//         container(create_note_btn).into()
//     }

//     // fn view(&self) -> Element<'_, Message> {
//     //     let create_note_btn = button("New").on_press(Message::CreateNote);

//     //     // container(column!([create_note_btn]))
//     //     // let side_menu =
//     //     // widget::text("Hello").into()
//     //     // create_note_btn.into()
//     //     // let mut widgets = vec![[]];
//     //     // // widgets.push(text("Hellot"));
//     //     // let create_note_btn = iced::widget::button(content)
//     //     // container(row!(widgets)).into()
//     // }
// }
/* #endregion */
