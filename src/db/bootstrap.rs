use rusqlite::{Connection, Result};

pub fn open_database(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Enforce SQLite invariants EARLY
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        ",
    )?;

    Ok(conn)
}