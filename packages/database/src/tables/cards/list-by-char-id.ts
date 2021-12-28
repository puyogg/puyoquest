import { db } from '../../db';
import { DatabaseError } from 'pg';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function listByCharId(params: {
  charId: string;
  includeMaterials?: boolean;
}): Promise<CardPublic[]> {
  const { charId } = params;

  try {
    const dbCards = await db.many<CardDb>('SELECT * FROM cards WHERE char_id = $1', [charId]);
    const publicCards = dbCards.map((card) => Util.camelCase<CardPublic>(card));
    return publicCards;
  } catch (err) {
    if (err instanceof DatabaseError) {
      throw Error(err.detail);
    } else {
      throw Error(`Failed to find cards matching charId: ${charId}`);
    }
  }
}
