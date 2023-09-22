/*
  Warnings:

  - You are about to drop the column `type` on the `Placeholder` table. All the data in the column will be lost.
  - Added the required column `placeholderType` to the `Placeholder` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "placeholderType" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    "selectedValueId" TEXT NOT NULL,
    CONSTRAINT "Placeholder_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Placeholder_selectedValueId_fkey" FOREIGN KEY ("selectedValueId") REFERENCES "PlaceholderValue" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "projectId", "selectedValueId") SELECT "id", "name", "projectId", "selectedValueId" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
