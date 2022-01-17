import { db } from '../../db';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function listRarestCharsByCharIds(charIds: string[]): Promise<CardPublic[]> {
  const dbCards = await db.many<CardDb>(
    `
    SELECT DISTINCT ON (char_id) *
    FROM cards
    WHERE char_id IN ($1:csv) AND card_type = 'character'
    ORDER BY char_id, rarity_modifier DESC NULLS LAST, rarity DESC;
    `,
    [charIds],
  );

  const publicCards = dbCards.map((card) => Util.camelCase<CardPublic>(card));
  return publicCards;
}
