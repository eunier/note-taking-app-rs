use cosmic::{
    iced::{Error, Sandbox, Settings},
    iced_widget::text,
};

fn main() -> Result<(), Error> {
    // Notes::run(Settings::default())
    Ok(())
}

// struct Notes;

// #[derive(Debug)]
// enum Message {}

// impl Sandbox for Notes {
//     type Message = Message;

//     fn new() -> Self {
//         todo!()
//     }

//     fn title(&self) -> String {
//         todo!()
//     }

//     fn update(&mut self, message: Self::Message) {
//         todo!()
//     }

//     fn view(
//         &self,
//         id: cosmic::iced_core::window::Id,
//     ) -> cosmic::iced::Element<'_, Self::Message> {
//         text("Hello").into()
//     }

//     fn close_requested(
//         &self,
//         id: cosmic::iced_core::window::Id,
//     ) -> Self::Message {
//         todo!()
//     }
// }
