import { db } from '../../db';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function listById(cardIds: string[]): Promise<CardPublic[]> {
  const cards = await db.many<CardDb>(
    `
    SELECT * FROM cards
    WHERE card_id IN ($1:csv)
    `,
    [cardIds],
  );

  return cards.map((card) => Util.camelCase<CardPublic>(card));
}
