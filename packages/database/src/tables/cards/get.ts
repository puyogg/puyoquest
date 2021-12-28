import { db } from '../../db';
import { DatabaseError } from 'pg';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function get(cardId: string): Promise<CardPublic> {
  try {
    const dbCard = await db.one<CardDb>('SELECT * FROM cards WHERE card_id = $1', [cardId]);
    const publicCard = Util.camelCase<CardPublic>(dbCard);
    return publicCard;
  } catch (err) {
    if (err instanceof DatabaseError) {
      throw Error(err.detail);
    } else {
      throw Error(`Failed to find cards matching cardId: ${cardId}`);
    }
  }
}
