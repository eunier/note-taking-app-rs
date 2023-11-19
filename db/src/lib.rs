pub use surrealdb;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::Thing;
use surrealdb::{Result, Surreal};

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub fn connect() {
    let binding = current_dir()
        .unwrap()
        .join("db/database/note_taking_add_dev");
    let path = binding.to_str().unwrap();
    let _ = DB.connect::<SpeeDb>(path);
}

#[derive(Debug, Serialize)]
struct Note {
    content: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct Database {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let a = DB.query("a");
        println!("{:?}", a);
        let path_buf = current_dir().unwrap();
        let current_dir_display = path_buf.display();
        let address =
            format!("{current_dir_display}/db/database/note_taking_add_dev");
        let db = Surreal::new::<SpeeDb>(address).await?;
        Ok(Self { db })
    }
}
