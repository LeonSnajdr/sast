/*
  Warnings:

  - You are about to drop the column `variety` on the `Task` table. All the data in the column will be lost.
  - You are about to drop the column `values` on the `Placeholder` table. All the data in the column will be lost.
  - You are about to drop the column `variety` on the `Placeholder` table. All the data in the column will be lost.
  - Made the column `value` on table `Placeholder` required. This step will fail if there are existing NULL values in that column.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "command" TEXT NOT NULL,
    "working_directory" TEXT NOT NULL,
    "delay" INTEGER NOT NULL,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Task" ("command", "delay", "id", "task_set_id", "working_directory") SELECT "command", "delay", "id", "task_set_id", "working_directory" FROM "Task";
DROP TABLE "Task";
ALTER TABLE "new_Task" RENAME TO "Task";
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "Placeholder_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "project_id", "value") SELECT "id", "name", "project_id", "value" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
