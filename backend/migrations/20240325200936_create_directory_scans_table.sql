-- Add migration script here
-- migrations/<timestamp>_create_directory_scans_table.sql
CREATE TABLE IF NOT EXISTS directory_scans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    root_path TEXT NOT NULL,
    scan_time TEXT NOT NULL
);
