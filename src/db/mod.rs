pub mod migrate;
pub mod executor;
pub mod bootstrap;
pub mod repositories;
pub mod seeder;

use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn init_database(path: &str) -> Result<Rc<RefCell<Connection>>, Box<dyn std::error::Error>> {
    let conn = Rc::new(RefCell::new(bootstrap::open_database(path)?));
    
    // Exécuter les migrations si nécessaire
    migrate::run_migrations(conn.clone(), vec![
        include_str!("../../sql/migrations/001_init.sql") // use include str to make sure migration actually exists
    ])?;
    
    // Exécuter le seeder si la base de données est vide
    seeder::run_seeder(conn.clone(), include_str!("../../sql/seeders/001_initial_seed.sql"))?;
    
    Ok(conn)
}
