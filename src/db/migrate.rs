use std::cell::RefCell;
use std::rc::Rc;
use rusqlite::Connection;
use crate::db::executor::execute_sql_script;

pub fn run_migrations(conn: Rc<RefCell<Connection>>, migrations: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    for migration in migrations {
        execute_sql_script(conn.clone(), migration)?;
    }

    Ok(())
}
