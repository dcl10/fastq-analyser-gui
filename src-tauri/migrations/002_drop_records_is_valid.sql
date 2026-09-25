-- Analysis now fails on invalid records instead of saving them, so every saved
-- record is valid and records.is_valid is no longer needed.

-- Remove invalid records saved before this change, which would otherwise show
-- up as valid once the column is gone.
DELETE FROM records WHERE is_valid = 0;

ALTER TABLE records DROP COLUMN is_valid;
