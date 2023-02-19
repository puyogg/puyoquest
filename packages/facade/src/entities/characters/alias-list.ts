import { Database } from '@ppq-wiki/database';

export async function aliasList(
  charId: string,
  includeMaterials = true,
): Promise<{ internalAliases: string[]; publicAliases: string[] }> {
  const aliases = await Database.Aliases.listCharacterAliases(charId, includeMaterials);
  return {
    internalAliases: aliases.filter((alias) => alias.internal).map((alias) => alias.alias),
    publicAliases: aliases.filter((alias) => !alias.internal).map((alias) => alias.alias),
  };
}
