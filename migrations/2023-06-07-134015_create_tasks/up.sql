-- Your SQL goes here
CREATE TABLE tasks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    due_date TIMESTAMP,
    status boolean DEFAULT false NOT NULL -- false = not done, true = done
);
