use std::io::{self, Write};

/// Prompt for login credentials
pub fn prompt_login() -> Result<(String, String), Box<dyn std::error::Error>> {
    print!("Login: ");
    io::stdout().flush()?;
    let mut login = String::new();
    io::stdin().read_line(&mut login)?;
    let login = login.trim().to_string();

    print!("Password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();

    Ok((login, password))
}

/// Attempt to login
pub fn login_attempt(app: &mut crate::app::App) -> Result<bool, Box<dyn std::error::Error>> {
    println!("\n=== Connexion ===");
    let (login, password) = prompt_login()?;

    match app.login(&login, &password) {
        Ok(true) => {
            println!("\n✓ Connexion réussie!\n");
            Ok(true)
        }
        Ok(false) => {
            println!("\n✗ Identifiants invalides.\n");
            Ok(false)
        }
        Err(e) => {
            println!("\n✗ Erreur: {}\n", e);
            Err(Box::new(e))
        }
    }
}

