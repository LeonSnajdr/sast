/*
  Warnings:

  - Added the required column `description` to the `TaskSet` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_TaskSet" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "TaskSet_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_TaskSet" ("id", "name", "project_id") SELECT "id", "name", "project_id" FROM "TaskSet";
DROP TABLE "TaskSet";
ALTER TABLE "new_TaskSet" RENAME TO "TaskSet";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
