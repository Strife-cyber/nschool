use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;
use crate::db::repositories::admin_repository::{AdminSession, Admin};
use crate::db::repositories::student_repository::StudentRepository;
use crate::db::repositories::subject_repository::SubjectRepository;
use crate::db::repositories::note_repository::NoteRepository;

/// Application state
pub struct App {
    pub conn: Rc<RefCell<Connection>>,
    pub current_admin: Option<Admin>,
    pub admin_session: AdminSession,
    pub student_repo: StudentRepository,
    pub subject_repo: SubjectRepository,
    pub note_repo: NoteRepository,
}

impl App {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Self {
        let admin_session = AdminSession::new(conn.clone());
        let student_repo = StudentRepository::new(conn.clone());
        let subject_repo = SubjectRepository::new(conn.clone());
        let note_repo = NoteRepository::new(conn.clone());

        Self {
            conn,
            current_admin: None,
            admin_session,
            student_repo,
            subject_repo,
            note_repo,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_admin.is_some()
    }

    pub fn login(&mut self, login: &str, password: &str) -> Result<bool, rusqlite::Error> {
        match self.admin_session.login(login, password)? {
            Some(admin) => {
                self.current_admin = Some(admin);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub fn logout(&mut self) {
        self.current_admin = None;
    }
}

