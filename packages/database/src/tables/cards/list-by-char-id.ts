import { db } from '../../db';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export async function listByCharId(params: {
  charId: string;
  includeMaterials?: boolean;
}): Promise<CardPublic[]> {
  const { charId } = params;

  const dbCards = await db.many<CardDb>('SELECT * FROM cards WHERE char_id = $1', [charId]);
  const publicCards = dbCards.map((card) => Util.camelCase<CardPublic>(card));
  return publicCards;
}
