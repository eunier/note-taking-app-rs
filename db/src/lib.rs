use serde::{Deserialize, Serialize};
use std::env::current_dir;
use surrealdb::engine::local::SpeeDb;
use surrealdb::sql::Thing;
use surrealdb::{Result, Surreal};

#[derive(Debug, Serialize)]
struct Note {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct Database {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let path_buf = current_dir().unwrap();
        let current_dir_display = path_buf.display();
        let address =
            format!("{current_dir_display}/db/database/note_taking_add_dev");
        let db = Surreal::new::<SpeeDb>(address).await?;
        Ok(Self { db })
    }
}
