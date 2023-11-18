use serde::{Deserialize, Serialize};
use std::env::current_dir;
use surrealdb::engine::local::SpeeDb;
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

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let binding = current_dir().unwrap();
    let current_dir_display = binding.display();
    let address =
        format!("{current_dir_display}/db/database/note_taking_add_dev");

    // Create database connection
    let db = Surreal::new::<SpeeDb>(address).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("person")
    //     .content(Person {
    //         title: "Founder & CEO",
    //         name: Name {
    //             first: "Tobie",
    //             last: "Morgan Hitchcock",
    //         },
    //         marketing: true,
    //     })
    //     .await?;
    // dbg!(created);

    // Update a person record with a specific id
    // let updated: Option<Record> = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(())
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
