import { db } from '../../db';
import { AliasDb } from './types';

export async function getByAlias(alias: string): Promise<AliasDb> {
  return db.one<AliasDb>(`SELECT * FROM aliases WHERE alias = $1`, [alias]);
}
