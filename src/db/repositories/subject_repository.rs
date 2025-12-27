use std::cell::RefCell;
use std::rc::Rc;
use crate::db::repositories::repository::Repository;
use rusqlite::{params, Connection, OptionalExtension, ToSql};

#[derive(Debug, Clone)]
pub struct Subject {
    pub code: String,
    pub name: String,
    pub class: String,
    pub coefficient: u8,
}

pub struct SubjectRepository {
    conn: Rc<RefCell<Connection>>
}

impl SubjectRepository {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Self {
        Self { conn }
    }

    //todo: implement relationships from a subject see students' notes
}

impl Repository<Subject> for SubjectRepository {
    fn create(&self, subject: &Subject) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
            "INSERT INTO subjects (code, name, class, coefficient) VALUES (?1, ?2, ?3, ?4)",
            params![subject.code, subject.name, subject.class, subject.coefficient]
        )?;
        Ok(())
    }

    fn get_all(&self) -> rusqlite::Result<Vec<Subject>> {
        let conn = self.conn.borrow();

        let mut stmt = conn.prepare(
            "SELECT code, name, class, coefficient FROM subjects"
        )?;

        let subjects = stmt.query_map([], |row| {
            Ok(Subject {
                code: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                coefficient: row.get(3)?,
            })
        })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(subjects)
    }

    fn get(&self, code: &str) -> rusqlite::Result<Option<Subject>> {
        let conn = self.conn.borrow();

        conn.query_row(
            "SELECT code, name, class, coefficient FROM subjects WHERE code = ?1",
            params![code],
            |row| Ok(Subject {
                code: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                coefficient: row.get(3)?
            })
        ).optional()
    }

    fn update(&self, subject: &Subject) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
            "UPDATE subjects set name = ?1, class = ?2, coefficient = ?3 WHERE code = ?4",
            params![subject.name, subject.class, subject.coefficient, subject.code]
        )?;
        Ok(())
    }

    fn delete(&self, code: &str) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
            "DELETE FROM subjects WHERE code = ?1",
            params![code]
        )?;
        Ok(())
    }

    fn filter(&self, filter: &str, params: &[&dyn ToSql]) -> rusqlite::Result<Vec<Subject>> {
        let conn = self.conn.borrow();

        let query = format!(
            "SELECT code, name, class, coefficient FROM subjects WHERE {}",
            filter
        );

        let mut stmt = conn.prepare(&query)?;
        let subjects = stmt.query_map(params, |row| {
            Ok(Subject {
                code: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                coefficient: row.get(3)?
            })
        })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(subjects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn setup_repo() -> SubjectRepository {
        let conn = Rc::new(RefCell::new(crate::db::bootstrap::open_database(":memory:").unwrap()));
        let schema = include_str!("../../../sql/schema.sql"); // make sure this exists
        crate::db::executor::execute_sql_script(conn.clone(), schema).unwrap();

        SubjectRepository::new(conn)
    }

    fn sample_subjects() -> Vec<Subject> {
        vec![
            Subject {
                code: "MATH101".into(),
                name: "Mathematics".into(),
                class: "10A".into(),
                coefficient: 4,
            },
            Subject {
                code: "PHY101".into(),
                name: "Physics".into(),
                class: "10A".into(),
                coefficient: 3,
            },
            Subject {
                code: "CHEM101".into(),
                name: "Chemistry".into(),
                class: "10B".into(),
                coefficient: 2,
            },
        ]
    }

    #[test]
    fn test_create_and_get_subject() {
        let repo = setup_repo();
        let subject = &sample_subjects()[0];

        repo.create(subject).unwrap();

        // get_all
        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].code, "MATH101");

        // get by code
        let fetched = repo.get("MATH101").unwrap().unwrap();
        assert_eq!(fetched.name, "Mathematics");

        // filtering
        let filtered = repo.filter("class = ?1", &[&"10A"]).unwrap();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_update_subject() {
        let repo = setup_repo();
        let mut subject = sample_subjects()[0].clone();
        repo.create(&subject).unwrap();

        subject.name = "Advanced Mathematics".into();
        subject.coefficient = 5;
        repo.update(&subject).unwrap();

        let updated = repo.get(&subject.code).unwrap().unwrap();
        assert_eq!(updated.name, "Advanced Mathematics");
        assert_eq!(updated.coefficient, 5);
    }

    #[test]
    fn test_delete_subject() {
        let repo = setup_repo();
        let subject = sample_subjects()[0].clone();
        repo.create(&subject).unwrap();

        repo.delete(&subject.code).unwrap();
        let deleted = repo.get(&subject.code).unwrap();
        assert!(deleted.is_none());
    }

    #[test]
    fn test_filter_subjects() {
        let repo = setup_repo();
        for subject in sample_subjects() {
            repo.create(&subject).unwrap();
        }

        // Filter class 10A
        let filtered = repo.filter("class = ?1", &[&"10A"]).unwrap();
        assert_eq!(filtered.len(), 2);

        // Filter non-existing class
        let filtered = repo.filter("class = ?1", &[&"12C"]).unwrap();
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_multiple_creates() {
        let repo = setup_repo();
        let subjects = sample_subjects();

        for subject in &subjects {
            repo.create(subject).unwrap();
        }

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 3);
    }
}

