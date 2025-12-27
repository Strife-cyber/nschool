/*
    ===================================================
    Migration Version : 001
    Created On        : 2025-01-27
    Author            : Strife-Cyber
    Description       : Initial schema for student
                        subject and note management
    ===================================================
*/

CREATE TABLE students (
/*
    The student "name" with surname "surname"
    enrolled in the class "class" and identified
    by the unique matricule "matricule".
*/
    matricule   VARCHAR(255) NOT NULL PRIMARY KEY,
    name        VARCHAR(60)  NOT NULL,
    surname     VARCHAR(60)  NOT NULL,
    class       VARCHAR(20)  NOT NULL
);

CREATE TABLE subjects (
/*
    The subject "name" identified by the code "code"
    taught in the class "class" and weighted by
    the coefficient "coefficient".
*/
    code        VARCHAR(60)  NOT NULL PRIMARY KEY,
    name        VARCHAR(60)  NOT NULL,
    class       VARCHAR(20)  NOT NULL,
    coefficient INTEGER      NOT NULL DEFAULT 1
);

CREATE TABLE notes (
/*
    The grade "value" obtained by the student
    identified by "matricule" for the subject
    identified by "subject_code".
*/
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    matricule    VARCHAR(255) NOT NULL,
    subject_code VARCHAR(60)  NOT NULL,
    value        REAL         NOT NULL,

    FOREIGN KEY (matricule) REFERENCES students(matricule),
    FOREIGN KEY (subject_code) REFERENCES subjects(code)
);
