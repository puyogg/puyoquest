import { Database } from '@ppq-wiki/database';
import { AliasPublic } from '@ppq-wiki/database/src/tables/aliases/types';

export async function aliasGet(alias: string): Promise<AliasPublic | null> {
  return Database.Aliases.get(alias);
}
