import { db } from '../../db';
import { AliasDb } from './types';

export async function list(): Promise<AliasDb[]> {
  const result = await db.manyOrNone<AliasDb>('SELECT * FROM aliases');
  return result;
}

// (async () => {
//   await list();
// })();
