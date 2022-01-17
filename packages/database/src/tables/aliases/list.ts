import { db } from '../../db';
import { AliasDb } from './types';

export async function list(aliases: string[]): Promise<string[]> {
  const result = await db.manyOrNone<Pick<AliasDb, 'char_id'>>(
    `SELECT char_id FROM aliases WHERE alias IN ($1:csv)`,
    [aliases],
  );

  const charIds = result.map((result) => result.char_id);
  return charIds;
}

// (async () => {
//   await list();
// })();
