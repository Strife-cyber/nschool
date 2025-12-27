use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use rusqlite::{Connection, Error as SqliteError};

#[derive(Debug)]
pub enum ScriptExecutionError {
    EmptyScript,
    MissingMigrationHeader,
    ForeignKeyPragmaFailed(SqliteError),
    TransactionBeginFailed(SqliteError),
    ScriptExecutionFailed(SqliteError),
    TransactionCommitFailed(SqliteError),
}

impl fmt::Display for ScriptExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScript =>
                write!(f, "SQL script is empty or contains only whitespace"),
            Self::MissingMigrationHeader =>
                write!(f, "SQL migration header is missing required metadata"),
            Self::ForeignKeyPragmaFailed(e) =>
                write!(f, "Failed to enable foreign keys: {e}"),
            Self::TransactionBeginFailed(e) =>
                write!(f, "Failed to start transaction: {e}"),
            Self::ScriptExecutionFailed(e) =>
                write!(f, "SQL script execution failed: {e}"),
            Self::TransactionCommitFailed(e) =>
                write!(f, "Failed to commit transaction: {e}"),
        }
    }
}

impl std::error::Error for ScriptExecutionError {}

fn validate_migration_header(script: &str) -> bool {
    script.contains("Migration Version")
        && script.contains("Created On")
        && script.contains("Author")
}

/// Executes a SQL script safely with validation and SQLite guarantees.
///
/// - Enables foreign keys
/// - Runs inside a transaction
/// - Supports multiple SQL statements
/// - Fails atomically on error
pub fn execute_sql_script(
    conn_ref: Rc<RefCell<Connection>>,
    script: &str,
) -> Result<(), ScriptExecutionError> {
    let trimmed = script.trim();

    // ---- Validation ----
    if trimmed.is_empty() {
        return Err(ScriptExecutionError::EmptyScript);
    }

    if !validate_migration_header(trimmed) {
        return Err(ScriptExecutionError::MissingMigrationHeader);
    }

    // Borrow mutably only for this block
    {
        let mut conn = conn_ref.borrow_mut();

        // ---- SQLite guarantees ----
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(ScriptExecutionError::ForeignKeyPragmaFailed)?;

        // ---- Atomic execution ----
        let tx = conn
            .transaction()
            .map_err(ScriptExecutionError::TransactionBeginFailed)?;

        tx.execute_batch(trimmed)
            .map_err(ScriptExecutionError::ScriptExecutionFailed)?;

        tx.commit()
            .map_err(ScriptExecutionError::TransactionCommitFailed)?;
    }

    Ok(())
}

