-- This file should undo anything in `up.sql`
ALTER TABLE "receipts" DROP COLUMN IF EXISTS transaction_id;