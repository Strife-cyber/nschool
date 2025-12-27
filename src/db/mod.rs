pub mod migrate;
pub mod executor;
pub mod bootstrap;
pub mod repositories;

use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn init_database(path: &str) -> Result<Rc<RefCell<Connection>>, Box<dyn std::error::Error>> {
    let conn = Rc::new(RefCell::new(bootstrap::open_database(path)?));
    migrate::run_migrations(conn.clone(), vec![
        include_str!("../../sql/migrations/001_init.sql") // use include str to make sure migration actually exists
    ])?;
    Ok(conn)
}
