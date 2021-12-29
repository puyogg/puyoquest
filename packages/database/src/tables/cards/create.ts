import { db } from '../../db';
import type { CardCreate, CardDb } from './types';

export async function create(params: CardCreate): Promise<number> {
  const insert: CardDb = {
    card_id: params.cardId,
    char_id: params.charId,
    rarity: params.rarity,
    name: params.name,
    name_normalized: params.nameNormalized,
    jp_name: params.jpName || null,
    link_name: params.linkName,
    card_type: params.cardType,
    updated_at: new Date(),
  };

  const result = await db.result(
    `
    INSERT INTO cards ($1:name)
    VALUES ($1:csv)
    ON CONFLICT (card_id)
    DO UPDATE SET
      char_id = EXCLUDED.char_id,
      rarity = EXCLUDED.rarity,
      name = EXCLUDED.name,
      name_normalized = EXCLUDED.name_normalized,
      jp_name = EXCLUDED.jp_name,
      link_name = EXCLUDED.link_name,
      card_type = EXCLUDED.card_type,
      updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );

  return result.rowCount;
}
