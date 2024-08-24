BEGIN;

CREATE OR REPLACE FUNCTION on_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.updated_at IS NULL
  THEN
    NEW.updated_at = now();
  END IF;

  RETURN NEW;
END;
$$ language 'plpgsql';

SET timezone = 'UTC';

COMMIT;