use comfy_table::Table;
use crate::db::repositories::student_repository::Student;
use crate::db::repositories::subject_repository::Subject;
use crate::db::repositories::note_repository::Note;

/// Display students in a table
pub fn display_students(students: &[Student]) {
    let mut table = Table::new();
    table.set_header(vec!["Matricule", "Nom", "Prénom", "Classe"]);

    for student in students {
        table.add_row(vec![
            &student.matricule,
            &student.name,
            &student.surname,
            &student.class,
        ]);
    }

    println!("{}", table);
}

/// Display subjects in a table
pub fn display_subjects(subjects: &[Subject]) {
    let mut table = Table::new();
    table.set_header(vec!["Code", "Nom", "Classe", "Coefficient"]);

    for subject in subjects {
        table.add_row(vec![
            &subject.code,
            &subject.name,
            &subject.class,
            &subject.coefficient.to_string(),
        ]);
    }

    println!("{}", table);
}

/// Display notes in a table
pub fn display_notes(notes: &[Note]) {
    let mut table = Table::new();
    table.set_header(vec!["ID", "Matricule", "Matière", "Note"]);

    for note in notes {
        table.add_row(vec![
            &note.id.to_string(),
            &note.matricule,
            &note.subject_code,
            &format!("{:.2}", note.value),
        ]);
    }

    println!("{}", table);
}

/// Display student notes with subject names (requires joining data)
pub fn display_student_notes_with_details(
    notes: &[Note],
    subjects: &[Subject],
    student: &Student,
) {
    let mut table = Table::new();
    table.set_header(vec!["Matière", "Code", "Note", "Coefficient"]);

    for note in notes {
        let subject = subjects.iter().find(|s| s.code == note.subject_code);
        let subject_name = subject
            .map(|s| s.name.as_str())
            .unwrap_or("Inconnu");
        let coefficient = subject
            .map(|s| s.coefficient.to_string())
            .unwrap_or_else(|| "?".to_string());

        table.add_row(vec![
            subject_name,
            &note.subject_code,
            &format!("{:.2}", note.value),
            &coefficient,
        ]);
    }

    println!("\nNotes pour: {} {} ({})", student.name, student.surname, student.matricule);
    println!("{}", table);
}

