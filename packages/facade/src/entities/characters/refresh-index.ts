import Axios from 'axios';
import { Database } from '@ppq-wiki/database';
import * as Util from '../../util';
import { WIKI_BASE_URL } from '../../constants';
import { CharacterCreate } from '@ppq-wiki/database/src/tables/characters';
import { CardCreate } from '@ppq-wiki/database/src/tables/cards';

export async function refreshIndex(
  charId: string,
): Promise<{ character: CharacterCreate; cards: CardCreate[] }> {
  const charTemplateRes = await Axios.get<string>(`${WIKI_BASE_URL}/Template:${charId}?action=raw`);
  const charData = Util.parseTemplate<Record<string, string>>(charTemplateRes.data);

  const charLinkName = charData['link'] || charData['name'];
  const charCreate: CharacterCreate = {
    charId,
    name: charData['name'],
    linkName: charLinkName,
    jpName: charData['jpname'],
    mainColor: charData['color'],
    sideColor: charData['color2'],
    type1: charData['type1'],
    type2: charData['type2'],
    voiceTrans: charData['voicetrans'],
  };
  await Database.Characters.create(charCreate);

  const cardKeys = Object.keys(charData).filter((key) => /card(\d+)|mat(\d+)/.test(key));
  const cardIds = cardKeys.map((key) => charData[key]);
  const cardData = await Promise.all(
    cardIds.map(async (cardId, i) => {
      const cardTemplateRes = await Axios.get<string>(
        `${WIKI_BASE_URL}/Template:${cardId}?action=raw`,
      );

      const card = Util.parseTemplate<Record<string, string>>(cardTemplateRes.data);
      const cardKey = cardKeys[i];
      const cardType = /card/i.test(cardKey) ? 'character' : 'material';

      const nameNormalized = Util.normalizeString(card['name']);
      const linkName = card['link'] || card['name'];
      const linkNameNormalized = Util.normalizeString(linkName);
      const rarityModifier = Util.parseRarityModifier(linkName);
      const jpNameNormalized = Util.normalizeString(card['jpname']);

      const databaseUpdates: Promise<unknown>[] = [];

      databaseUpdates.push(
        Database.Aliases.upsert({
          alias: nameNormalized,
          charId,
          internal: true,
          cardType,
        }),
      );

      databaseUpdates.push(
        Database.Aliases.upsert({
          alias: linkNameNormalized,
          charId,
          internal: true,
          cardType,
        }),
      );

      if (jpNameNormalized) {
        databaseUpdates.push(
          Database.Aliases.upsert({
            alias: jpNameNormalized,
            charId,
            internal: true,
            cardType,
          }),
        );
      }

      const cardDatum: CardCreate = {
        cardId: card['code'],
        charId,
        rarity: card['rarity'],
        rarityModifier,
        name: card['name'],
        nameNormalized,
        jpName: card['jpname'],
        jpNameNormalized,
        linkName,
        linkNameNormalized,
        cardType: cardType as 'character' | 'material',
      };
      databaseUpdates.push(Database.Cards.upsert(cardDatum));

      await Promise.all(databaseUpdates);
      return cardDatum;
    }),
  );

  return {
    character: charCreate,
    cards: cardData,
  };
}
