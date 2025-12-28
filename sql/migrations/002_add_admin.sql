/*
    ===================================================
    Migration Version : 001
    Created On        : 2025-01-28
    Author            : Strife-Cyber
    Description       : Addition of admin for management
    ===================================================
*/

CREATE TABLE admins (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    login           VARCHAR(25) NOT NULL UNIQUE,
    password        VARCHAR(25) NOT NULL
);
