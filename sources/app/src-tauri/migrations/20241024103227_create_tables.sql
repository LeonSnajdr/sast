CREATE TABLE IF NOT EXISTS project (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    date_created TEXT NOT NULL,
    date_last_opened TEXT NOT NULL
);

CREATE UNIQUE INDEX uidx_name ON project(name);
