use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::sync::Arc;
pub use surrealdb;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize)]
struct Note {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Error {
    Message(String),
}

// TODO remove all expect
pub async fn create_note() -> Result<Vec<Record>, Error> {
    let db = connect().await?;

    let res: Vec<Record> = db
        .create("note")
        .content(Note {
            content: "Lorem ipsum dolor sit amet, consectetur duis reprehenderit dolore sunt deserunt dolore irure ipsum ex".to_string(),
        })
        .await
        .map_err(|err| Error::Message(err.to_string()))
        .expect("should create a note");

    dbg!("created note", &res);
    Ok(res)
}

pub async fn connect() -> Result<Surreal<Db>, Error> {
    let binding = current_dir().unwrap();
    let current_dir_display = binding.display();
    let address =
        format!("{current_dir_display}/db/database/note_taking_add_dev");

    // Create database connection
    let db = Surreal::new::<SpeeDb>(address)
        .await
        .map_err(|err| Error::Message(err.to_string()))
        .expect("should be able to connect to database");

    // Select a specific namespace / database
    db.use_ns("test")
        .use_db("test")
        .await
        .map_err(|err| Error::Message(err.to_string()))
        .expect("schould be able to select a namespace and database");

    dbg!("connected to database");
    Ok(db)
}
