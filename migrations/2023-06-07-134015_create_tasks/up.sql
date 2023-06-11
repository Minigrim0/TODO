-- Your SQL goes here
CREATE TABLE tasks (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    due_date TIMESTAMP,
    status boolean DEFAULT false NOT NULL -- false = not done, true = done
);
