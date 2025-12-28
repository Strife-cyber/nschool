use std::cell::RefCell;
use std::rc::Rc;
use rusqlite::Connection;
use crate::db::executor::execute_sql_script;

/// Vérifie si une table existe dans la base de données
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
        [table_name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Exécute les migrations uniquement si les tables n'existent pas encore
pub fn run_migrations(conn: Rc<RefCell<Connection>>, migrations: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Vérifier si les tables principales existent déjà
    let tables_exist = {
        let conn_ref = conn.borrow();
        table_exists(&conn_ref, "students")? 
            && table_exists(&conn_ref, "subjects")? 
            && table_exists(&conn_ref, "notes")?
            && table_exists(&conn_ref, "admins")?
    };

    if tables_exist {
        // Les tables existent déjà, ne pas exécuter les migrations
        return Ok(());
    }

    // Exécuter les migrations
    for migration in migrations {
        execute_sql_script(conn.clone(), migration)?;
    }

    Ok(())
}
