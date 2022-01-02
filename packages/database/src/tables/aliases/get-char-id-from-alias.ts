import { db } from '../../db';
import { AliasDb } from './types';

export async function getCharIdFromAlias(alias: string): Promise<string> {
  const result = await db.one<Pick<AliasDb, 'char_id'>>(
    `SELECT char_id FROM aliases WHERE alias = $1`,
    [alias],
  );
  return result.char_id;
}
