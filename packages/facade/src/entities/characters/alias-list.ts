import { Database } from '@ppq-wiki/database';

export async function aliasList(charId: string): Promise<string[]> {
  return Database.Aliases.listCharacterAliases(charId);
}
