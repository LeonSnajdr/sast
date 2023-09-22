/*
  Warnings:

  - You are about to drop the column `placeholderValue` on the `Placeholder` table. All the data in the column will be lost.
  - You are about to drop the column `placeholderValues` on the `Placeholder` table. All the data in the column will be lost.
  - Added the required column `variety` to the `Placeholder` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "variety" TEXT NOT NULL,
    "value" TEXT,
    "values" TEXT,
    "projectId" TEXT NOT NULL,
    CONSTRAINT "Placeholder_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "projectId") SELECT "id", "name", "projectId" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
