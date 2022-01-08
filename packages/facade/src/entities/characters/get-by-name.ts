import { Database } from '@ppq-wiki/database';
import { CardPublic } from '@ppq-wiki/database/src/tables/cards';
import { CharacterPublic } from '@ppq-wiki/database/src/tables/characters';
import { Util } from '../../util';

export async function getByName(params: { name: string; includeMaterials: boolean }): Promise<{
  character: CharacterPublic;
  cards: CardPublic[];
}> {
  const { name, includeMaterials } = params;

  const normalizedInputName = Util.normalizeString(name);
  const charId = await Database.Aliases.getCharIdFromAlias(normalizedInputName);

  const character = await Database.Characters.get(charId);
  const cards = await Database.Cards.listByCharId({ charId, includeMaterials });

  return {
    character,
    cards,
  };
}
