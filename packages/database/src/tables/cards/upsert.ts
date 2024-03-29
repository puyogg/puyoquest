import { db } from '../../db';
import type { CardCreate, CardDb } from './types';

export async function upsert(params: CardCreate): Promise<number> {
  const insert: CardDb = {
    card_id: params.cardId,
    char_id: params.charId,
    rarity: params.rarity,
    rarity_modifier: params.rarityModifier || null,
    name: params.name,
    name_normalized: params.nameNormalized,
    jp_name: params.jpName || null,
    jp_name_normalized: params.jpNameNormalized || null,
    link_name: params.linkName,
    link_name_normalized: params.linkNameNormalized,
    card_type: params.cardType,
    main_color: params.mainColor,
    side_color: params.sideColor,
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
      rarity_modifier = EXCLUDED.rarity_modifier,
      name = EXCLUDED.name,
      name_normalized = EXCLUDED.name_normalized,
      jp_name = EXCLUDED.jp_name,
      jp_name_normalized = EXCLUDED.jp_name_normalized,
      link_name = EXCLUDED.link_name,
      link_name_normalized = EXCLUDED.link_name_normalized,
      card_type = EXCLUDED.card_type,
      main_color = EXCLUDED.main_color,
      side_color = EXCLUDED.side_color,
      updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );

  return result.rowCount;
}
