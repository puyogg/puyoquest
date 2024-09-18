CREATE TYPE card_type AS ENUM ('character', 'material');

BEGIN;
ALTER TABLE aliases RENAME TO alias;
ALTER TABLE alias ALTER COLUMN card_type TYPE card_type using card_type::card_type;
ALTER TABLE alias ALTER COLUMN char_id SET NOT NULL;
ALTER TABLE alias ALTER COLUMN internal SET NOT NULL;
ALTER TABLE alias ALTER COLUMN card_type SET NOT NULL;
ALTER TABLE alias ALTER COLUMN updated_at SET NOT NULL;
COMMIT;

BEGIN;
UPDATE cards
SET main_color = 'Blue'
WHERE char_id = '2422';

UPDATE cards
SET side_color = 'Red'
WHERE char_id = '2422' and rarity = '6' and rarity_modifier = '6-2' and card_type = 'character';

UPDATE cards
SET main_color = 'Red'
WHERE card_id = '156116';
COMMIT;

BEGIN;
ALTER TABLE cards RENAME TO card;
ALTER TABLE card ALTER COLUMN card_type TYPE card_type using card_type::card_type;
ALTER TABLE card ALTER COLUMN char_id SET NOT NULL;
ALTER TABLE card ALTER COLUMN rarity SET NOT NULL;
ALTER TABLE card ALTER COLUMN name SET NOT NULL;
ALTER TABLE card ALTER COLUMN name_normalized SET NOT NULL;
ALTER TABLE card ALTER COLUMN link_name SET NOT NULL;
ALTER TABLE card ALTER COLUMN link_name_normalized SET NOT NULL;
ALTER TABLE card ALTER COLUMN card_type SET NOT NULL;
ALTER TABLE card ALTER COLUMN main_color SET NOT NULL;
ALTER TABLE card ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE card ADD wiki_template JSONB;
COMMIT;

BEGIN;
ALTER TABLE characters RENAME TO character;
ALTER TABLE character ALTER COLUMN updated_at SET NOT NULL;
COMMIT;
