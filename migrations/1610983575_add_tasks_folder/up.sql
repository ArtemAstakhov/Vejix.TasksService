DROP TABLE IF EXISTS tasks_lists;

CREATE TABLE tasks_lists (
  id SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL,
  user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);

ALTER TABLE tasks
ADD COLUMN task_list_id INTEGER DEFAULT NULL REFERENCES tasks_lists (id) ON DELETE CASCADE;