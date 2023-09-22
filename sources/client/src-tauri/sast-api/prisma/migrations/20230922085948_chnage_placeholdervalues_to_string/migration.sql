/*
  Warnings:

  - You are about to drop the `PlaceholderValue` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the column `placeholderType` on the `Placeholder` table. All the data in the column will be lost.
  - You are about to drop the column `selectedValueId` on the `Placeholder` table. All the data in the column will be lost.
  - Added the required column `placeholderValue` to the `Placeholder` table without a default value. This is not possible if the table is not empty.
  - Added the required column `placeholderValues` to the `Placeholder` table without a default value. This is not possible if the table is not empty.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "PlaceholderValue";
PRAGMA foreign_keys=on;

-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Placeholder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "placeholderValue" TEXT NOT NULL,
    "placeholderValues" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    CONSTRAINT "Placeholder_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Placeholder" ("id", "name", "projectId") SELECT "id", "name", "projectId" FROM "Placeholder";
DROP TABLE "Placeholder";
ALTER TABLE "new_Placeholder" RENAME TO "Placeholder";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
