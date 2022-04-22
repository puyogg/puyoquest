import { db } from '../../db';
import { AliasDb } from './types';

export async function listCharacterAliases(charId: string): Promise<string[]> {
  const result = await db.manyOrNone<Pick<AliasDb, 'alias'>>(
    'SELECT alias FROM aliases WHERE char_id = $1',
    [charId],
  );

  const aliases = result.map((result) => result.alias);
  return aliases;
}
