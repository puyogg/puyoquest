import { db } from '../../db';
import { AliasCreate, AliasDb } from './types';

export async function create(params: AliasCreate): Promise<number> {
  const { alias, charId } = params;

  const updatedAt = new Date();

  const insert: AliasDb = {
    alias,
    char_id: charId,
    updated_at: updatedAt,
  };

  const result = await db.result('INSERT INTO aliases($1:name) VALUES($1:csv)', [insert]);
  return result.rowCount;
}
