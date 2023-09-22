-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "placeholderType" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    "selectedValueId" TEXT,
    CONSTRAINT "Placeholder_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Placeholder_selectedValueId_fkey" FOREIGN KEY ("selectedValueId") REFERENCES "PlaceholderValue" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "placeholderType", "projectId", "selectedValueId") SELECT "id", "name", "placeholderType", "projectId", "selectedValueId" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
