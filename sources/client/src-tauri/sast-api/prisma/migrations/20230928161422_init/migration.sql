-- CreateTable
CREATE TABLE "Project" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "variety" TEXT NOT NULL,
    "value" TEXT,
    "values" TEXT,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "Placeholder_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "TaskSet" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "TaskSet_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "type" TEXT NOT NULL,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
