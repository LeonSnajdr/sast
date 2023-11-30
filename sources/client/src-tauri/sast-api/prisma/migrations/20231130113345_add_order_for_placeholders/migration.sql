/*
  Warnings:

  - Added the required column `order` to the `Placeholder` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "order" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "project_id" TEXT NOT NULL,
    CONSTRAINT "Placeholder_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "Project" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "project_id", "value") SELECT "id", "name", "project_id", "value" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
CREATE UNIQUE INDEX "Placeholder_project_id_name_key" ON "Placeholder"("project_id", "name");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
