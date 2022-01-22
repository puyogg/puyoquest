import { Database } from '@ppq-wiki/database';
import * as Util from '../../util';
import { getLore, WikiLore } from './get-lore';

export async function getLoreByNameAndRarity(params: {
  name: string;
  rarity: string;
}): Promise<WikiLore> {
  const { rarity, rarityModifier } = Util.parseInputRarity(params.rarity);

  const normalizedInputName = Util.normalizeString(params.name);
  const charId = await Database.Aliases.getCharIdFromAlias(normalizedInputName);

  const cardPublic = await Database.Cards.getByCharIdAndRarity({
    charId,
    rarity,
    rarityModifier,
  });

  const { cardId } = cardPublic;
  return getLore(cardId);
}
