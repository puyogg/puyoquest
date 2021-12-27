import { db } from '../../db';

export async function deleteByCharId(charId: string): Promise<number> {
  const result = await db.result('DELETE FROM aliases WHERE char_id = $1', [charId]);
  return result.rowCount;
}

// (async () => {
//   await deleteByAlias('S2LSOFTENER');
// })();
