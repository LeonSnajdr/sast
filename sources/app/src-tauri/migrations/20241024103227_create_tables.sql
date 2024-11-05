CREATE TABLE IF NOT EXISTS project (
    id UUID NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX uidx_name ON project(name);
