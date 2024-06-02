BEGIN;

CREATE OR REPLACE FUNCTION on_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ language 'plpgsql';

SET timezone = 'UTC';

COMMIT;