/*
    ===================================================
    Seeder Version : 002
    Created On        : 2025-01-28
    Author            : Strife-Cyber
    Description       : Seed 10 admins for testing
    ===================================================
*/

-- Utiliser INSERT OR IGNORE pour éviter les doublons si certains admins existent déjà
INSERT OR IGNORE INTO admins(login, password) VALUES 
    ('admin', 'admin'),
    ('admin1', 'password1'),
    ('admin2', 'password2'),
    ('admin3', 'password3'),
    ('admin4', 'password4'),
    ('admin5', 'password5'),
    ('admin6', 'password6'),
    ('admin7', 'password7'),
    ('admin8', 'password8'),
    ('admin9', 'password9');
