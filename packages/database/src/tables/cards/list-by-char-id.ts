import { db } from '../../db';
import * as Util from '../../util';
import type { CardDb, CardPublic } from './types';

export const LIST_BY_CHAR_ID_QUERIES = {
  INCLUDE_MATERIALS: `SELECT * FROM cards WHERE char_id = $1 AND (card_type = 'character' OR card_type = 'material')`,
  CHARACTERS_ONLY: `SELECT * FROM cards WHERE char_id = $1 AND card_type = 'character'`,
};

export async function listByCharId(params: {
  charId: string;
  includeMaterials?: boolean;
}): Promise<CardPublic[]> {
  const { charId, includeMaterials } = params;

  const query = includeMaterials
    ? LIST_BY_CHAR_ID_QUERIES.INCLUDE_MATERIALS
    : LIST_BY_CHAR_ID_QUERIES.CHARACTERS_ONLY;

  const dbCards = await db.many<CardDb>(query, [charId]);
  const publicCards = dbCards.map((card) => Util.camelCase<CardPublic>(card));
  return publicCards;
}
