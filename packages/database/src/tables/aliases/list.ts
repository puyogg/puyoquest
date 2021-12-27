import { db } from '../../db';
import { DbAlias } from './create';

export async function list(): Promise<void> {
  const result = await db.manyOrNone<DbAlias>('SELECT * FROM aliases');
  console.log(result);
}

// (async () => {
//   await list();
// })();
