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
        include_str!("../../sql/migrations/001_init.sql"), // use include str to make sure migration actually exists
        include_str!("../../sql/migrations/002_add_admin.sql") // include admins during migration
    ])?;
    
    // Exécuter le seeder si la base de données est vide
    seeder::run_seeder(conn.clone(), include_str!("../../sql/seeders/001_initial_seed.sql"))?;
    // Toujours s'assurer que les admins sont présents
    seeder::run_seeder_force(conn.clone(), include_str!("../../sql/seeders/002_admin_seed.sql"))?;
    
    Ok(conn)
}
