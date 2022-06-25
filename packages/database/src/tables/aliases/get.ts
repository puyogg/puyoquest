import { db } from '../../db';
import * as Util from '../../util';
import { AliasDb, AliasPublic } from './types';

export async function get(alias: string): Promise<AliasPublic | null> {
  const aliasDb = await db.oneOrNone<AliasDb>('SELECT * FROM aliases WHERE alias = $1', [alias]);

  if (!aliasDb) {
    return null;
  }

  const aliasPublic = Util.camelCase<AliasPublic>(aliasDb);

  return aliasPublic;
}
