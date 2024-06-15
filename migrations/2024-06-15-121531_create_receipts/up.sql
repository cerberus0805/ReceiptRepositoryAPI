-- Your SQL goes here
CREATE TABLE "receipts" (
  "id" SERIAL PRIMARY KEY,
  "transaction_date" TIMESTAMP NOT NULL,
  "is_inventory_taxed" Boolean NOT NULL,
  "currency_id" INTEGER NOT NULL,
  "store_id" INTEGER NOT NULL,
  FOREIGN KEY ("currency_id") references currencies(id),
  FOREIGN KEY ("store_id") references stores(id)
);