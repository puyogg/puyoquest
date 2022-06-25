import { Database } from '@ppq-wiki/database';

export async function aliasList(
  charId: string,
): Promise<{ internalAliases: string[]; publicAliases: string[] }> {
  const aliases = await Database.Aliases.listCharacterAliases(charId);
  return {
    internalAliases: aliases.filter((alias) => alias.internal).map((alias) => alias.alias),
    publicAliases: aliases.filter((alias) => !alias.internal).map((alias) => alias.alias),
  };
}
