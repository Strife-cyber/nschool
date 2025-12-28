mod db;
mod app;
mod auth;
mod cli;
mod views;

use std::io;
use crate::db::init_database;
use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================================");
    println!("🎓 Nschool - Student Management System");
    println!("=======================================\n");

    // Initialize database
    let conn = init_database("database/nschool.sqlite")?;
    println!("✓ Base de données initialisée\n");

    // Create app instance
    let mut app = App::new(conn);

    // Main application loop
    loop {
        if !app.is_authenticated() {
            // Login required
            match auth::login_attempt(&mut app) {
                Ok(true) => continue, // Login successful, show menu
                Ok(false) => continue, // Login failed, try again
                Err(e) => {
                    eprintln!("Erreur: {}", e);
                    continue;
                }
            }
        } else {
            // Show main menu
            cli::show_main_menu();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();

            match cli::handle_main_command(&mut app, choice) {
                Ok(true) => continue, // Continue loop
                Ok(false) => break,    // Exit application
                Err(e) => {
                    eprintln!("\n✗ Erreur: {}\n", e);
                    continue;
                }
            }
        }
    }

    Ok(())
}
