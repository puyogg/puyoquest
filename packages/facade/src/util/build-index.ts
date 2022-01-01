import { WIKI_BASE_URL } from '../constants';
import { Util } from './index';
import * as pMap from 'p-map';
import Axios from 'axios';
import { Logger } from '../logger';
import { parseTemplate } from './parse-template';
import { Database } from '@ppq-wiki/database';

export async function buildIndex(): Promise<void> {
  const categoryPages = [
    'Category:PPQ:Red_Color',
    'Category:PPQ:Blue_Color',
    'Category:PPQ:Green_Color',
    'Category:PPQ:Yellow_Color',
    'Category:PPQ:Purple_Color',
  ];

  const linkNameSet = new Set<string>();

  await pMap(
    categoryPages,
    async (categoryPage) => {
      const linkNames = await Util.getAllCategoryLinkNames(categoryPage);

      for (const linkName of linkNames) {
        try {
          if (linkNameSet.has(linkName)) {
            return;
          }
          // Ignore Category:PPQ:Red_Color/Cards subcategory
          else if (/\/Cards$/.test(linkName)) {
            return;
          }
          linkNameSet.add(linkName);

          const charPageUrl = `${WIKI_BASE_URL}/PPQ:${linkName}?action=raw`;
          const charPageRes = await Axios.get<string>(charPageUrl);

          const charPageRaw = charPageRes.data;
          // Ex. {{184442L|long}} or {{2012|long}}
          // Capture the character id string.
          const charIdMatch = charPageRaw.match(/(\{\{)(\w+?)\|/);
          if (!charIdMatch) return;
          const charId = charIdMatch[2].trim();

          const charTemplatePageUrl = `${WIKI_BASE_URL}/Template:${charId}?action=raw`;
          const charTemplatePageRes = await Axios.get<string>(charTemplatePageUrl);

          const charTemplate = charTemplatePageRes.data;
          const characterData = parseTemplate(charTemplate);
          await Database.Characters.create({
            charId,
            name: characterData['name'],
            jpName: characterData['jpname'],
            mainColor: characterData['color'],
            sideColor: characterData['color2'],
            type1: characterData['type1'],
            type2: characterData['type2'],
            voiceTrans: characterData['voicetrans'],
          });

          const cardKeys = Object.keys(characterData).filter((key) =>
            /card(\d+)|mat(\d+)/.test(key),
          );
          const cardIds = cardKeys.map((key) => characterData[key]);
          await Promise.all(
            cardIds.map(async (cardId, j) => {
              const cardTemplatePageUrl = `${WIKI_BASE_URL}/Template:${cardId}?action=raw`;
              const cardTemplatePageRes = await Axios.get<string>(cardTemplatePageUrl);
              Logger.AxiosResponse(cardTemplatePageRes);

              const cardTemplate = cardTemplatePageRes.data;
              const parsedCard = parseTemplate(cardTemplate);

              const cardKey = cardKeys[j];
              const cardType = /card/i.test(cardKey) ? 'character' : 'material';

              const linkName = parsedCard['link'] || parsedCard['name'];
              const rarityModifier = Util.parseRarityModifier(linkName);

              await Database.Cards.create({
                cardId: parsedCard['code'],
                charId,
                rarity: parsedCard['rarity'],
                rarityModifier,
                name: parsedCard['name'],
                nameNormalized: Util.normalizeString(parsedCard['name']),
                jpName: parsedCard['jpname'],
                jpNameNormalized: Util.normalizeString(parsedCard['jpname']),
                linkName,
                linkNameNormalized: Util.normalizeString(linkName),
                cardType,
              });
            }),
          );
        } catch (err) {
          console.error(linkName, err);
        }
      }
    },
    { concurrency: 1 },
  );
}

(async () => {
  try {
    await buildIndex();
  } catch (err) {
    console.error(err);
  }
})();
