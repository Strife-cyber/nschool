mod db;
mod ui;

use crate::db::init_database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _conn = init_database("database/nschool.sqlite")?;

    Ok(())
}
