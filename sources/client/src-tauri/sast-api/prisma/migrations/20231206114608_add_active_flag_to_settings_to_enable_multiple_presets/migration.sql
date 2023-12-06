/*
  Warnings:

  - The primary key for the `Settings` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - Added the required column `active` to the `Settings` table without a default value. This is not possible if the table is not empty.
  - Added the required column `name` to the `Settings` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Settings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "active" BOOLEAN NOT NULL,
    "language" TEXT NOT NULL,
    "theme" TEXT NOT NULL,
    "default_dir" TEXT NOT NULL
);
INSERT INTO "new_Settings" ("default_dir", "id", "language", "theme") SELECT "default_dir", "id", "language", "theme" FROM "Settings";
DROP TABLE "Settings";
ALTER TABLE "new_Settings" RENAME TO "Settings";
CREATE UNIQUE INDEX "Settings_name_key" ON "Settings"("name");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
