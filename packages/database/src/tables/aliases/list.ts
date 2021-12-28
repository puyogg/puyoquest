import { db } from '../../db';
import { AliasDb } from './create';

export async function list(): Promise<void> {
  const result = await db.manyOrNone<AliasDb>('SELECT * FROM aliases');
  console.log(result);
}

// (async () => {
//   await list();
// })();
