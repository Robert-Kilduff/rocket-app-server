-- Your SQL goes here
CREATE TABLE habits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    max_progress_per_period INTEGER,
    FOREIGN KEY (user_id) REFERENCES Users(id)
)


