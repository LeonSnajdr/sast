/*
  Warnings:

  - Made the column `command` on table `Task` required. This step will fail if there are existing NULL values in that column.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "variety" TEXT NOT NULL,
    "command" TEXT NOT NULL,
    "delay" INTEGER NOT NULL DEFAULT 0,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Task" ("command", "delay", "id", "task_set_id", "variety") SELECT "command", coalesce("delay", 0) AS "delay", "id", "task_set_id", "variety" FROM "Task";
DROP TABLE "Task";
ALTER TABLE "new_Task" RENAME TO "Task";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
