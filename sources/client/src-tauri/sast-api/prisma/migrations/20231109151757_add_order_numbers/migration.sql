/*
  Warnings:

  - Added the required column `order` to the `TaskSet` table without a default value. This is not possible if the table is not empty.
  - Added the required column `order` to the `Task` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_TaskSet" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "order" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "TaskSet_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
INSERT INTO "new_TaskSet" ("description", "id", "name", "project_id") SELECT "description", "id", "name", "project_id" FROM "TaskSet";
DROP TABLE "TaskSet";
ALTER TABLE "new_TaskSet" RENAME TO "TaskSet";
CREATE UNIQUE INDEX "TaskSet_project_id_name_key" ON "TaskSet"("project_id", "name");
CREATE TABLE "new_Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "order" INTEGER NOT NULL,
    "command" TEXT NOT NULL,
    "working_directory" TEXT NOT NULL,
    "delay" INTEGER NOT NULL,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
INSERT INTO "new_Task" ("command", "delay", "id", "task_set_id", "working_directory") SELECT "command", "delay", "id", "task_set_id", "working_directory" FROM "Task";
DROP TABLE "Task";
ALTER TABLE "new_Task" RENAME TO "Task";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
