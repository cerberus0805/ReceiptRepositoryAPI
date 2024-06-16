-- Your SQL goes here
CREATE TABLE "currencies" (
  "id" SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL
);

CREATE TABLE "stores" (
  "id" SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL,
  "alias" TEXT,
  "branch" TEXT,
  "address" TEXT
);

CREATE TABLE "products" (
  "id" SERIAL PRIMARY KEY,
  "name" TEXT NOT NULL,
  "alias" TEXT,
  "brand" TEXT,
  "specification_amount" INTEGER,
  "specification_unit" TEXT,
  "specification_others" TEXT
);
