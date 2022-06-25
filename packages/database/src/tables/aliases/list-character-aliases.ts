import { db } from '../../db';
import { AliasDb } from './types';

export async function listCharacterAliases(
  charId: string,
): Promise<{ alias: string; internal: boolean }[]> {
  const result = await db.manyOrNone<Pick<AliasDb, 'alias' | 'internal'>>(
    'SELECT alias, internal FROM aliases WHERE char_id = $1',
    [charId],
  );

  return result;
}
