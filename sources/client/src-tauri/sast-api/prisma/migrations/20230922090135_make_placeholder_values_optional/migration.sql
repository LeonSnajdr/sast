-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "placeholderValue" TEXT,
    "placeholderValues" TEXT,
    "projectId" TEXT NOT NULL,
    CONSTRAINT "Placeholder_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "placeholderValue", "placeholderValues", "projectId") SELECT "id", "name", "placeholderValue", "placeholderValues", "projectId" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
