use std::cell::RefCell;
use std::rc::Rc;
use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug, Clone, PartialEq)]
pub struct Admin {
    pub id: i64,
    pub login: String,
    pub password: String, // plain for now, hash later
}

pub struct AdminSession {
    conn: Rc<RefCell<Connection>>,
}

impl AdminSession {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Self {
        Self { conn }
    }

    /// Try to authenticate an admin.
    /// Returns:
    /// - Ok(Some(Admin)) if credentials are valid
    /// - Ok(None) if invalid
    /// - Err if DB error
    pub fn login(
        &self,
        login: &str,
        password: &str,
    ) -> rusqlite::Result<Option<Admin>> {
        let conn = self.conn.borrow();

        conn.query_row(
            "SELECT id, login, password FROM admins WHERE login = ?1 AND password = ?2",
            params![login, password],
            |row| {
                Ok(Admin {
                    id: row.get(0)?,
                    login: row.get(1)?,
                    password: row.get(2)?,
                })
            },
        )
            .optional()
    }

    /// Simple helper for tests / setup
    pub fn create(&self, admin: &Admin) -> rusqlite::Result<()> {
        let conn = self.conn.borrow_mut();
        conn.execute(
            "INSERT INTO admins (login, password) VALUES (?1, ?2)",
            params![admin.login, admin.password],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use rusqlite::params;

    fn setup_session() -> AdminSession {
        let conn = Rc::new(RefCell::new(
            crate::db::bootstrap::open_database(":memory:").unwrap(),
        ));

        let schema = include_str!("../../../sql/schema.sql");
        crate::db::executor::execute_sql_script(conn.clone(), schema).unwrap();

        AdminSession::new(conn)
    }

    fn sample_admin() -> Admin {
        Admin {
            id: 1,
            login: "admin".into(),
            password: "secret".into(),
        }
    }

    #[test]
    fn test_successful_login() {
        let session = setup_session();
        let admin = sample_admin();

        session.create(&admin).unwrap();

        let logged = session.login("admin", "secret").unwrap();
        assert!(logged.is_some());

        let logged = logged.unwrap();
        assert_eq!(logged.login, "admin");
    }

    #[test]
    fn test_wrong_password() {
        let session = setup_session();
        let admin = sample_admin();

        session.create(&admin).unwrap();

        let logged = session.login("admin", "wrong").unwrap();
        assert!(logged.is_none());
    }

    #[test]
    fn test_unknown_user() {
        let session = setup_session();

        let logged = session.login("ghost", "secret").unwrap();
        assert!(logged.is_none());
    }

    #[test]
    fn test_multiple_admins() {
        let session = setup_session();

        session.create(&Admin {
            id: 1,
            login: "admin1".into(),
            password: "pw1".into(),
        }).unwrap();

        session.create(&Admin {
            id: 2,
            login: "admin2".into(),
            password: "pw2".into(),
        }).unwrap();

        assert!(session.login("admin1", "pw1").unwrap().is_some());
        assert!(session.login("admin2", "pw2").unwrap().is_some());
        assert!(session.login("admin1", "pw2").unwrap().is_none());
    }
}
