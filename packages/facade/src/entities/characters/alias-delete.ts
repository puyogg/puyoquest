import { Database } from '@ppq-wiki/database';

export async function aliasDelete(alias: string): Promise<number> {
  return Database.Aliases.deleteByAlias(alias);
}
