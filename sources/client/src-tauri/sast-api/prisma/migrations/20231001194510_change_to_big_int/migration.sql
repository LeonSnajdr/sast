/*
  Warnings:

  - You are about to alter the column `delay` on the `Task` table. The data in that column could be lost. The data in that column will be cast from `Int` to `BigInt`.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "variety" TEXT NOT NULL,
    "command" TEXT NOT NULL,
    "delay" BIGINT NOT NULL DEFAULT 0,
    "task_set_id" TEXT NOT NULL,
    CONSTRAINT "Task_task_set_id_fkey" FOREIGN KEY ("task_set_id") REFERENCES "TaskSet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Task" ("command", "delay", "id", "task_set_id", "variety") SELECT "command", "delay", "id", "task_set_id", "variety" FROM "Task";
DROP TABLE "Task";
ALTER TABLE "new_Task" RENAME TO "Task";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
