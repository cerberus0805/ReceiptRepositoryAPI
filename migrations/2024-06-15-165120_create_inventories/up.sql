-- Your SQL goes here




CREATE TABLE "inventories"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"price" NUMERIC NOT NULL,
	"quantity" INT4 NOT NULL,
	"product_id" INT4 NOT NULL,
	"receipt_id" INT4 NOT NULL,
	FOREIGN KEY ("product_id") REFERENCES "products"("id"),
	FOREIGN KEY ("receipt_id") REFERENCES "receipts"("id")
);

