-- Your SQL goes here
ALTER TABLE "receipts" ADD COLUMN IF NOT EXISTS transaction_id UUID;