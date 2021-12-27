// TODO refactor with pg-promise instead
import { DbPool } from '../../db-pool';

export interface Card {
  /** Primary Key */
  cardId: string;
  charId: string;
  rarity: string;
  name: string;
  nameNormalized: string;
  jpName?: string;
  linkName: string;
  cardType: 'character' | 'material';
}

export async function create(params: Card): Promise<void> {
  const {
    cardId,
    charId,
    rarity,
    name,
    nameNormalized,
    jpName = null,
    linkName,
    cardType,
  } = params;

  const updatedAt = new Date();

  const client = await DbPool.connect();
  try {
    await client.query(
      `
      INSERT INTO cards(card_id, char_id, rarity, name, name_normalized, jp_name, link_name, card_type, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
      [cardId, charId, rarity, name, nameNormalized, jpName, linkName, cardType, updatedAt],
    );
  } finally {
    client.release();
  }
}
