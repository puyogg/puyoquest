import { db } from '../../db';
import { AliasCreate, AliasDb } from './types';

export async function upsert(params: AliasCreate): Promise<number> {
  const { alias, charId, internal, cardType } = params;

  const updatedAt = new Date();

  const insert: AliasDb = {
    alias,
    char_id: charId,
    internal,
    card_type: cardType,
    updated_at: updatedAt,
  };

  const result = await db.result(
    `
    INSERT INTO aliases($1:name) VALUES($1:csv)
    ON CONFLICT (alias)
    DO UPDATE SET
      char_id = EXCLUDED.char_id,
      internal = EXCLUDED.internal,
      updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );
  return result.rowCount;
}
