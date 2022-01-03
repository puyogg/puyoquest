import { Database } from '@ppq-wiki/database';
import { Util } from '../../util';
import { getWikiCard } from './get-wiki-card';

export async function getByNameAndRarity(params: {
  /** Could be the non-normalized name, non-normalized link name, or the Japanese name */
  name: string;
  /** Needs to be normalized to a valid rarity */
  rarity: string;
}) {
  const { rarity, rarityModifier } = Util.parseInputRarity(params.rarity);

  const normalizedInputName = Util.normalizeString(params.name);
  const charId = await Database.Aliases.getCharIdFromAlias(normalizedInputName);

  const cardPublic = await Database.Cards.getByCharIdAndRarity({
    charId,
    rarity,
    rarityModifier,
  });
  const { cardId } = cardPublic;
  return getWikiCard(cardId);
}
