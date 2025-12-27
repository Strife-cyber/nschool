use std::rc::Rc;
use std::cell::RefCell;
use crate::db::repositories::repository::Repository;
use rusqlite::{params, Connection, OptionalExtension, ToSql};

#[derive(Debug, Clone)]
pub struct Student {
    pub matricule: String,
    pub name: String,
    pub surname: String,
    pub class: String,
}

pub struct StudentRepository {
    conn: Rc<RefCell<Connection>>
}

impl StudentRepository {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Self {
        Self { conn }
    }

    //todo: implement relationships from a student see his notes in diff subjects
}

impl Repository<Student> for StudentRepository {
    fn create(&self, student: &Student) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();

        conn.execute(
           "INSERT INTO students (matricule, name, surname, class) VALUES (?1, ?2, ?3, ?4)",
           params![student.matricule, student.name, student.surname, student.class]
        )?;
        Ok(())
    }

    fn get_all(&self) -> rusqlite::Result<Vec<Student>> {
        let conn = self.conn.borrow();
        let mut stmt = conn.prepare(
            "SELECT matricule, name, surname, class FROM students"
        )?;

        let students = stmt.query_map([], |row| {
            Ok(Student {
                matricule: row.get(0)?,
                name: row.get(1)?,
                surname: row.get(2)?,
                class: row.get(3)?,
            })
        })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(students)
    }

    fn get(&self, matricule: &str) -> rusqlite::Result<Option<Student>> {
        let conn = self.conn.borrow();
        conn.query_row(
            "SELECT matricule, name, surname, class FROM students WHERE matricule = ?1",
            params![matricule],
            |row| Ok(Student {
                matricule: row.get(0)?,
                name: row.get(1)?,
                surname: row.get(2)?,
                class: row.get(3)?
            })
        ).optional()
    }

    fn update(&self, student: &Student) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();
        conn.execute(
            "UPDATE students SET name=?1, surname=?2, class=?3 WHERE matricule=?4",
            params![student.name, student.surname, student.class, student.matricule],
        )?;
        Ok(())
    }

    fn delete(&self, matricule: &str) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();
        conn.execute(
            "DELETE FROM students WHERE matricule=?1",
            params![matricule],
        )?;
        Ok(())
    }

    fn filter(&self, filter: &str, params: &[&dyn ToSql]) -> rusqlite::Result<Vec<Student>> {
        let conn = self.conn.borrow();

        let query = format!(
            "SELECT matricule, name, surname, class FROM students WHERE {}",
            filter
        );

        let mut stmt = conn.prepare(&query)?;
        let students = stmt.query_map(params, |row| {
            Ok(Student {
                matricule: row.get(0)?,
                name: row.get(1)?,
                surname: row.get(2)?,
                class: row.get(3)?,
            })
        })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(students)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn setup_repo() -> StudentRepository {
        let conn = Rc::new(RefCell::new(crate::db::bootstrap::open_database(":memory:").unwrap()));
        let schema = include_str!("../../../sql/schema.sql");
        crate::db::executor::execute_sql_script(conn.clone(), schema).unwrap();

        StudentRepository::new(conn)
    }

    fn sample_students() -> Vec<Student> {
        vec![
            Student {
                matricule: "S001".into(),
                name: "Alice".into(),
                surname: "Smith".into(),
                class: "10A".into(),
            },
            Student {
                matricule: "S002".into(),
                name: "Bob".into(),
                surname: "Johnson".into(),
                class: "10B".into(),
            },
            Student {
                matricule: "S003".into(),
                name: "Charlie".into(),
                surname: "Brown".into(),
                class: "10A".into(),
            },
        ]
    }

    #[test]
    fn test_create_and_get_student() {
        let repo = setup_repo();
        let student = &sample_students()[0];

        repo.create(student).unwrap();

        // get_all
        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].matricule, "S001");

        // get by matricule
        let fetched = repo.get("S001").unwrap().unwrap();
        assert_eq!(fetched.name, "Alice");

        // filtering
        let filtered = repo.filter("class = ?1", &[&"10A"]).unwrap();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_update_student() {
        let repo = setup_repo();
        let mut student = sample_students()[0].clone();
        repo.create(&student).unwrap();

        student.name = "Alicia".into();
        student.class = "11A".into();
        repo.update(&student).unwrap();

        let updated = repo.get(&student.matricule).unwrap().unwrap();
        assert_eq!(updated.name, "Alicia");
        assert_eq!(updated.class, "11A");
    }

    #[test]
    fn test_delete_student() {
        let repo = setup_repo();
        let student = sample_students()[0].clone();
        repo.create(&student).unwrap();

        repo.delete(&student.matricule).unwrap();
        let deleted = repo.get(&student.matricule).unwrap();
        assert!(deleted.is_none());
    }

    #[test]
    fn test_filter_students() {
        let repo = setup_repo();
        for student in sample_students() {
            repo.create(&student).unwrap();
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
        let students = sample_students();

        for student in &students {
            repo.create(student).unwrap();
        }

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 3);
    }
}
