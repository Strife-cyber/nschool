mod db;

use crate::db::init_database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = init_database("database/nschool.sqlite")?;

    // From here on:
    // - DB is ready
    // - Schema exists
    // - PRAGMA are enforced

    Ok(())
}
