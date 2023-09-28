/*
  Warnings:

  - You are about to drop the column `type` on the `Task` table. All the data in the column will be lost.
  - Added the required column `variety` to the `Task` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "variety" TEXT NOT NULL,
    "command" TEXT,
    "delay" INTEGER,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Task" ("command", "delay", "id", "task_set_id") SELECT "command", "delay", "id", "task_set_id" FROM "Task";
DROP TABLE "Task";
ALTER TABLE "new_Task" RENAME TO "Task";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
