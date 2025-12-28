use std::io::{self, Write};
use crate::app::App;
use crate::db::repositories::repository::Repository;

/// Main menu for authenticated users
pub fn show_main_menu() {
    println!("\n=== Menu Principal ===");
    println!("1.  Voir tous les étudiants");
    println!("2.  Voir un étudiant (par matricule)");
    println!("3.  Voir toutes les matières");
    println!("4.  Voir une matière (par code)");
    println!("5.  Voir toutes les notes");
    println!("6.  Voir les notes d'un étudiant");
    println!("7.  Voir les notes d'une matière");
    println!("8.  Déconnexion");
    println!("9.  Quitter");
    print!("\nChoisissez une option: ");
    io::stdout().flush().unwrap();
}

/// Handle main menu commands
pub fn handle_main_command(app: &mut App, choice: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match choice {
        "1" => {
            println!("\n=== Tous les étudiants ===");
            let students = app.student_repo.get_all()?;
            if students.is_empty() {
                println!("Aucun étudiant trouvé.\n");
            } else {
                crate::views::display_students(&students);
                println!();
            }
            Ok(true)
        }
        "2" => {
            print!("\nMatricule: ");
            io::stdout().flush()?;
            let mut matricule = String::new();
            io::stdin().read_line(&mut matricule)?;
            let matricule = matricule.trim();

            match app.student_repo.get(matricule)? {
                Some(student) => {
                    println!("\n=== Étudiant ===");
                    crate::views::display_students(&[student.clone()]);
                    
                    // Show student notes
                    let notes = app.note_repo.get_by_student(matricule)?;
                    if !notes.is_empty() {
                        let subjects = app.subject_repo.get_all()?;
                        crate::views::display_student_notes_with_details(&notes, &subjects, &student);
                    }
                    println!();
                }
                None => {
                    println!("\n✗ Étudiant non trouvé.\n");
                }
            }
            Ok(true)
        }
        "3" => {
            println!("\n=== Toutes les matières ===");
            let subjects = app.subject_repo.get_all()?;
            if subjects.is_empty() {
                println!("Aucune matière trouvée.\n");
            } else {
                crate::views::display_subjects(&subjects);
                println!();
            }
            Ok(true)
        }
        "4" => {
            print!("\nCode de la matière: ");
            io::stdout().flush()?;
            let mut code = String::new();
            io::stdin().read_line(&mut code)?;
            let code = code.trim();

            match app.subject_repo.get(code)? {
                Some(subject) => {
                    println!("\n=== Matière ===");
                    crate::views::display_subjects(&[subject.clone()]);
                    
                    // Show subject notes
                    let notes = app.note_repo.get_by_subject(code)?;
                    if !notes.is_empty() {
                        println!("\n=== Notes pour cette matière ===");
                        crate::views::display_notes(&notes);
                    }
                    println!();
                }
                None => {
                    println!("\n✗ Matière non trouvée.\n");
                }
            }
            Ok(true)
        }
        "5" => {
            println!("\n=== Toutes les notes ===");
            let notes = app.note_repo.get_all()?;
            if notes.is_empty() {
                println!("Aucune note trouvée.\n");
            } else {
                crate::views::display_notes(&notes);
                println!();
            }
            Ok(true)
        }
        "6" => {
            print!("\nMatricule de l'étudiant: ");
            io::stdout().flush()?;
            let mut matricule = String::new();
            io::stdin().read_line(&mut matricule)?;
            let matricule = matricule.trim();

            match app.student_repo.get(matricule)? {
                Some(student) => {
                    let notes = app.note_repo.get_by_student(matricule)?;
                    if notes.is_empty() {
                        println!("\nAucune note trouvée pour cet étudiant.\n");
                    } else {
                        let subjects = app.subject_repo.get_all()?;
                        crate::views::display_student_notes_with_details(&notes, &subjects, &student);
                        println!();
                    }
                }
                None => {
                    println!("\n✗ Étudiant non trouvé.\n");
                }
            }
            Ok(true)
        }
        "7" => {
            print!("\nCode de la matière: ");
            io::stdout().flush()?;
            let mut code = String::new();
            io::stdin().read_line(&mut code)?;
            let code = code.trim();

            match app.subject_repo.get(code)? {
                Some(_) => {
                    let notes = app.note_repo.get_by_subject(code)?;
                    if notes.is_empty() {
                        println!("\nAucune note trouvée pour cette matière.\n");
                    } else {
                        println!("\n=== Notes pour la matière {} ===", code);
                        crate::views::display_notes(&notes);
                        println!();
                    }
                }
                None => {
                    println!("\n✗ Matière non trouvée.\n");
                }
            }
            Ok(true)
        }
        "8" => {
            app.logout();
            println!("\n✓ Déconnexion réussie.\n");
            Ok(true)
        }
        "9" => {
            println!("\nAu revoir!");
            Ok(false)
        }
        _ => {
            println!("\n✗ Option invalide.\n");
            Ok(true)
        }
    }
}

