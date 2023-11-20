use serde::{Deserialize, Serialize};
use std::env::current_dir;
pub use surrealdb;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize)]
struct Note {
    content: String,
}

pub enum Error {
    Message(String),
}

pub async fn create_note(
    db: &Surreal<Db>,
) -> Result<Vec<Record>, surrealdb::Error> {
    // let created: Vec<Record> = db
    // .create("person")
    // .content(Person {
    //     title: "Founder & CEO",
    //     name: Name {
    //         first: "Tobie",
    //         last: "Morgan Hitchcock",
    //     },
    //     marketing: true,
    // })
    // .await?;

    // return

    let a: Vec<Record> = db
        .create("note")
        .content(Note {
            content: String::new(),
        })
        .await?;

    let db_call: Vec<Record> = db
        .create("note")
        .content(Note {
            content: String::new(),
        })
        .await;

    // let b: Vec<Record> = match db
    //     .create("note")
    //     .content(Note {
    //         content: String::new(),
    //     }).await {

    //     }

    let notes: Vec<Record> = db.select("note").await?;
    dbg!(notes);

    // db.select("note").await?
    Ok(a)
}

pub async fn connect() -> Result<Surreal<Db>, surrealdb::Error> {
    let binding = current_dir().unwrap();
    let current_dir_display = binding.display();
    let address =
        format!("{current_dir_display}/db/database/note_taking_add_dev");

    // Create database connection
    let db = Surreal::new::<SpeeDb>(address).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(db)
}

// use once_cell::sync::Lazy;
// use serde::{Deserialize, Serialize};
// use std::env::current_dir;
// pub use surrealdb;
// use surrealdb::engine::local::{Db, SpeeDb};
// use surrealdb::sql::Thing;
// use surrealdb::{Result, Surreal};

// static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

// #[derive(Debug, Serialize)]
// struct Name<'a> {
//     first: &'a str,
//     last: &'a str,
// }

// #[derive(Debug, Serialize)]
// struct Person<'a> {
//     title: &'a str,
//     name: Name<'a>,
//     marketing: bool,
// }

// #[derive(Debug, Serialize)]
// struct Responsibility {
//     marketing: bool,
// }

// #[derive(Debug, Deserialize)]
// struct Record {
//     #[allow(dead_code)]
//     id: Thing,
// }

// pub async fn connect() -> Result<()> {
//     let binding = current_dir()
//         .unwrap()
//         .join("db/database/note_taking_add_dev");

//     let path = binding.to_str().unwrap();
//     let _ = DB.connect::<SpeeDb>(path);
//     DB.use_ns("test").use_db("test").await?;

//     DB = Surreal::new::<SpeeDb>(path).await;

//     // Create a new person with a random id
//     let created: Vec<Record> = DB
//         .create("person")
//         .content(Person {
//             title: "Founder & CEO",
//             name: Name {
//                 first: "Tobie",
//                 last: "Morgan Hitchcock",
//             },
//             marketing: true,
//         })
//         .await?;
//     dbg!(created);

//     // Update a person record with a specific id
//     let updated: Option<Record> = DB
//         .update(("person", "jaime"))
//         .merge(Responsibility { marketing: true })
//         .await?;
//     dbg!(updated);

//     // Select all people records
//     let people: Vec<Record> = DB.select("person").await?;
//     dbg!(people);

//     // Perform a custom advanced query
//     let groups = DB
//         .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
//         .bind(("table", "person"))
//         .await?;
//     dbg!(groups);

//     Ok(())
// }

// // #[derive(Debug, Serialize)]
// // struct Note {
// //     content: String,
// // }

// // #[derive(Clone, Debug, Deserialize)]
// // struct Record {
// //     #[allow(dead_code)]
// //     id: Thing,
// // }

// // pub struct Database {
// //     db: Surreal<surrealdb::engine::local::Db>,
// // }

// // impl Database {
// //     pub async fn new() -> Result<Self> {
// //         let a = DB.query("a");
// //         println!("{:?}", a);
// //         let path_buf = current_dir().unwrap();
// //         let current_dir_display = path_buf.display();
// //         let address =
// //             format!("{current_dir_display}/db/database/note_taking_add_dev");
// //         let db = Surreal::new::<SpeeDb>(address).await?;
// //         Ok(Self { db })
// //     }
// // }
