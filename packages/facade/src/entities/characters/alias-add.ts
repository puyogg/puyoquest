import { Database } from '@ppq-wiki/database';

export async function aliasAdd(params: { charId: string; alias: string }): Promise<number> {
  const { charId, alias } = params;

  return Database.Aliases.upsert({
    alias,
    charId,
    internal: false,
    cardType: 'character',
  });
}
