use std::cell::RefCell;
use std::rc::Rc;
use crate::db::repositories::repository::Repository;
use rusqlite::{params, Connection, OptionalExtension, ToSql};

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub value: f64,
    pub matricule: String,
    pub subject_code: String,
}

pub struct NoteRepository {
    conn: Rc<RefCell<Connection>>,
}

impl NoteRepository {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Self {
        NoteRepository { conn }
    }
}

impl  Repository<Note> for NoteRepository {
    fn create(&self, note: &Note) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
            "INSERT INTO notes (value, matricule, subject_code) VALUES (?1, ?2, ?3)",
            params![note.value, note.matricule, note.subject_code]
        )?;
        Ok(())
    }

    fn get_all(&self) -> rusqlite::Result<Vec<Note>> {
        let conn = self.conn.borrow();
        let mut stmt = conn.prepare(
            "SELECT id, value, matricule, subject_code FROM notes"
        )?;

        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                value: row.get(1)?,
                matricule: row.get(2)?,
                subject_code: row.get(3)?,
            })
        })?
            .collect::<rusqlite::Result<Vec<Note>>>()?;

        Ok(notes)
    }

    fn get(&self, id: &str) -> rusqlite::Result<Option<Note>> {
        let conn = self.conn.borrow();

        conn.query_row(
            "SELECT id, value, matricule, subject_code FROM notes WHERE id = ?1",
            params![id],
            |row| Ok(Note{
                id: row.get(0)?,
                value: row.get(1)?,
                matricule: row.get(2)?,
                subject_code: row.get(3)?,
            })
        ).optional()
    }

    fn update(&self, item: &Note) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
            "UPDATE notes SET value=?1, matricule=?2, subject_code=?3 WHERE id = ?4",
            params![item.value, item.matricule, item.subject_code, item.id]
        )?;
        Ok(())
    }

    fn delete(&self, id: &str) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();
        conn.execute(
            "DELETE FROM notes WHERE id = ?1",
            params![id]
        )?;
        Ok(())
    }

    fn filter(&self, filter: &str, params: &[&dyn ToSql]) -> rusqlite::Result<Vec<Note>> {
        let conn = self.conn.borrow();

        let query = format!(
            "SELECT id, value, matricule, subject_code FROM notes WHERE {}",
            filter
        );

        let mut stmt = conn.prepare(&query)?;
        let notes = stmt.query_map(params, |row| {
            Ok(Note {
                id: row.get(0)?,
                value: row.get(1)?,
                matricule: row.get(2)?,
                subject_code: row.get(3)?,
            })
        })?
            .collect::<rusqlite::Result<Vec<Note>>>()?;

        Ok(notes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn setup_repo() -> NoteRepository {
        let conn = Rc::new(RefCell::new(crate::db::bootstrap::open_database(":memory:").unwrap()));
        let schema = include_str!("../../../sql/schema.sql");
        crate::db::executor::execute_sql_script(conn.clone(), schema).unwrap();

        // Insert all students referenced in sample_notes
        conn.borrow_mut().execute(
            "INSERT INTO students (matricule, name, surname, class) VALUES (?1, ?2, ?3, ?4)",
            params!["S001", "Alice", "Smith", "10A"]
        ).unwrap();
        conn.borrow_mut().execute(
            "INSERT INTO students (matricule, name, surname, class) VALUES (?1, ?2, ?3, ?4)",
            params!["S002", "Bob", "Jones", "10A"]
        ).unwrap();

        // Insert all subjects referenced in sample_notes and test_update_note
        conn.borrow_mut().execute(
            "INSERT INTO subjects (code, name, class, coefficient) VALUES (?1, ?2, ?3, ?4)",
            params!["MATH101", "Mathematics", "10A", 4]
        ).unwrap();
        conn.borrow_mut().execute(
            "INSERT INTO subjects (code, name, class, coefficient) VALUES (?1, ?2, ?3, ?4)",
            params!["PHY101", "Physics", "10A", 3]
        ).unwrap();
        conn.borrow_mut().execute(
            "INSERT INTO subjects (code, name, class, coefficient) VALUES (?1, ?2, ?3, ?4)",
            params!["CHEM101", "Chemistry", "10A", 2]
        ).unwrap();

        NoteRepository::new(conn)
    }

    fn sample_notes() -> Vec<Note> {
        vec![
            Note {
                id: 1,
                value: 15f64,
                matricule: "S001".into(),
                subject_code: "MATH101".into(),
            },
            Note {
                id: 2,
                value: 12f64,
                matricule: "S002".into(),
                subject_code: "PHY101".into(),
            },
            Note {
                id: 3,
                value: 18f64,
                matricule: "S001".into(),
                subject_code: "PHY101".into(),
            },
        ]
    }

    #[test]
    fn test_create_and_get_note() {
        let repo = setup_repo();
        let note = &sample_notes()[0];

        repo.create(note).unwrap();

        // get_all
        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].matricule, "S001");
        assert_eq!(all[0].value, 15f64);

        // get by id
        let fetched = repo.get(&note.id.to_string()).unwrap().unwrap();
        assert_eq!(fetched.subject_code, "MATH101");

        // filter by matricule
        let filtered = repo.filter("matricule = ?1", &[&"S001"]).unwrap();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_update_note() {
        let repo = setup_repo();
        let mut note = sample_notes()[0].clone();
        repo.create(&note).unwrap();

        note.value = 17f64;
        note.subject_code = "CHEM101".into();
        repo.update(&note).unwrap();

        let updated = repo.get(&note.id.to_string()).unwrap().unwrap();
        assert_eq!(updated.value, 17f64);
        assert_eq!(updated.subject_code, "CHEM101");
    }

    #[test]
    fn test_delete_note() {
        let repo = setup_repo();
        let note = sample_notes()[0].clone();
        repo.create(&note).unwrap();

        repo.delete(&note.id.to_string()).unwrap();
        let deleted = repo.get(&note.id.to_string()).unwrap();
        assert!(deleted.is_none());
    }

    #[test]
    fn test_filter_notes() {
        let repo = setup_repo();
        for note in sample_notes() {
            repo.create(&note).unwrap();
        }

        // filter by matricule S001
        let filtered = repo.filter("matricule = ?1", &[&"S001"]).unwrap();
        assert_eq!(filtered.len(), 2);

        // filter by subject_code PHY101
        let filtered = repo.filter("subject_code = ?1", &[&"PHY101"]).unwrap();
        assert_eq!(filtered.len(), 2);

        // filter non-existing
        let filtered = repo.filter("value > ?1", &[&20]).unwrap();
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_multiple_creates() {
        let repo = setup_repo();
        let notes = sample_notes();

        for note in &notes {
            repo.create(note).unwrap();
        }

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 3);
    }
}

