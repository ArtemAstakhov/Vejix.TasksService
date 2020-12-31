DROP TABLE IF EXISTS checklists;

CREATE TABLE checklists (
  id SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL,
  user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);

DROP TABLE IF EXISTS checklist_items;

CREATE TABLE checklist_items (
  id SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL,
  completed boolean NOT NULL DEFAULT false,
  checklist_id INTEGER NOT NULL REFERENCES checklists (id) ON DELETE CASCADE,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);
