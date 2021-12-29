import { db } from '../../db';

export async function deleteByAlias(alias: string): Promise<number> {
  const result = await db.result('DELETE FROM aliases WHERE alias = $1', [alias]);
  return result.rowCount;
}
