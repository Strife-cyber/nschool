use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use rusqlite::{Connection, Error as SqliteError};

#[derive(Debug)]
pub enum SeederError {
    EmptyScript,
    ForeignKeyPragmaFailed(SqliteError),
    TransactionBeginFailed(SqliteError),
    ScriptExecutionFailed(SqliteError),
    TransactionCommitFailed(SqliteError),
}

impl fmt::Display for SeederError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScript =>
                write!(f, "SQL seeder script is empty or contains only whitespace"),
            Self::ForeignKeyPragmaFailed(e) =>
                write!(f, "Failed to enable foreign keys: {e}"),
            Self::TransactionBeginFailed(e) =>
                write!(f, "Failed to start transaction: {e}"),
            Self::ScriptExecutionFailed(e) =>
                write!(f, "SQL seeder script execution failed: {e}"),
            Self::TransactionCommitFailed(e) =>
                write!(f, "Failed to commit transaction: {e}"),
        }
    }
}

impl std::error::Error for SeederError {}

/// Vérifie si la base de données contient déjà des données
pub fn has_data(conn: &Connection) -> Result<bool, rusqlite::Error> {
    // Vérifier si la table students existe et contient des données
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='students'",
        [],
        |row| row.get(0),
    )?;

    if count == 0 {
        return Ok(false);
    }

    // Vérifier si des étudiants existent déjà
    let student_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM students",
        [],
        |row| row.get(0),
    )?;

    Ok(student_count > 0)
}

/// Exécute un script SQL de seeding de manière sécurisée
fn execute_seeder_script(
    conn_ref: Rc<RefCell<Connection>>,
    script: &str,
) -> Result<(), SeederError> {
    let trimmed = script.trim();

    // Validation
    if trimmed.is_empty() {
        return Err(SeederError::EmptyScript);
    }

    // Exécution sécurisée
    {
        let mut conn = conn_ref.borrow_mut();

        // Activer les clés étrangères
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(SeederError::ForeignKeyPragmaFailed)?;

        // Exécution atomique dans une transaction
        let tx = conn
            .transaction()
            .map_err(SeederError::TransactionBeginFailed)?;

        tx.execute_batch(trimmed)
            .map_err(SeederError::ScriptExecutionFailed)?;

        tx.commit()
            .map_err(SeederError::TransactionCommitFailed)?;
    }

    Ok(())
}

/// Exécute le script de seeding si la base de données est vide
pub fn run_seeder(
    conn: Rc<RefCell<Connection>>,
    seeder_script: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Vérifier si des données existent déjà
    let has_existing_data = {
        let conn_ref = conn.borrow();
        has_data(&conn_ref)?
    };

    if has_existing_data {
        // Les données existent déjà, ne pas exécuter le seeder
        return Ok(());
    }

    // Exécuter le script de seeding
    execute_seeder_script(conn, seeder_script)?;

    Ok(())
}

