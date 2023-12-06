-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Settings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "default" BOOLEAN NOT NULL DEFAULT false,
    "active" BOOLEAN NOT NULL,
    "language" TEXT NOT NULL,
    "theme" TEXT NOT NULL,
    "default_dir" TEXT NOT NULL
);
INSERT INTO "new_Settings" ("active", "default_dir", "id", "language", "name", "theme") SELECT "active", "default_dir", "id", "language", "name", "theme" FROM "Settings";
DROP TABLE "Settings";
ALTER TABLE "new_Settings" RENAME TO "Settings";
CREATE UNIQUE INDEX "Settings_name_key" ON "Settings"("name");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
