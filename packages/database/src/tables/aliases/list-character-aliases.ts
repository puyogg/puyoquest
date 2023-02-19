import { db } from '../../db';
import { AliasDb } from './types';

export async function listCharacterAliases(
  charId: string,
  includeMaterials = true,
): Promise<{ alias: string; internal: boolean }[]> {
  const query = includeMaterials
    ? 'SELECT alias, internal FROM aliases WHERE char_id = $1'
    : `SELECT alias, internal FROM aliases WHERE char_id = $1 AND card_type = 'character'`;
  const result = await db.manyOrNone<Pick<AliasDb, 'alias' | 'internal'>>(query, [charId]);

  return result;
}
