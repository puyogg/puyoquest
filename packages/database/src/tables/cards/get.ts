import { db } from '../../db';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function get(cardId: string): Promise<CardPublic> {
  const dbCard = await db.one<CardDb>('SELECT * FROM cards WHERE card_id = $1', [cardId]);
  const publicCard = Util.camelCase<CardPublic>(dbCard);
  return publicCard;
}
