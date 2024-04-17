-- Your SQL goes here
CREATE TABLE task_habit (
    task_id INTEGER NOT NULL,
    habit_id INTEGER NOT NULL,
    contribution INTEGER,
    PRIMARY KEY (task_id, habit_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (habit_id) REFERENCES habits(id)
)
