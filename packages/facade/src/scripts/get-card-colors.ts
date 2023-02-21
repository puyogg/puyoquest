import Axios from 'axios';
import { Db } from '@ppq-wiki/database';
import * as Util from '../util';
import { Database } from '@ppq-wiki/database';
import { WIKI_BASE_URL } from '../constants';

type CardDb = Database.Cards.CardDb;

async function fetchTemplate(id: string): Promise<string> {
  const templateUrl = `${WIKI_BASE_URL}/Template:${id}?action=raw`;
  const templateRes = await Axios.get<string>(templateUrl);
  return templateRes.data;
}

async function getCardColors() {
  const rows = await Db.db.many<CardDb>(
    'SELECT * FROM cards WHERE main_color is null ORDER BY card_id',
  );
  const total = rows.length;

  while (rows.length > 0) {
    const card = rows.shift();
    if (!card) return;

    try {
      const template = await fetchTemplate(card.card_id);
      const cardData = Util.parseTemplate<Record<string, string>>(template);

      const nameNormalized = Util.normalizeString(cardData['name']);
      const linkName = cardData['link'] || cardData['name'];
      const linkNameNormalized = Util.normalizeString(linkName);
      const rarityModifier = Util.parseRarityModifier(linkName);
      const jpNameNormalized = Util.normalizeString(cardData['jpname']);

      console.log(
        `[${total - rows.length}/${total}] Updating ${cardData['name']} ${
          cardData['rarity']
        } with main: ${cardData['color']}, side: ${cardData['color2']}`,
      );

      await Promise.race([
        Database.Cards.upsert({
          cardId: cardData['code'],
          charId: card.char_id,
          rarity: cardData['rarity'],
          rarityModifier,
          name: cardData['name'],
          nameNormalized,
          jpName: cardData['jpname'],
          jpNameNormalized,
          linkName,
          linkNameNormalized,
          cardType: card.card_type,
          mainColor: cardData['color'],
          sideColor: cardData['color2'],
        }),
        new Promise((resolve, reject) => setTimeout(reject, 10000)),
      ]);
      // await Database.Cards.upsert({
      //   cardId: cardData['code'],
      //   charId: card.char_id,
      //   rarity: cardData['rarity'],
      //   rarityModifier,
      //   name: cardData['name'],
      //   nameNormalized,
      //   jpName: cardData['jpname'],
      //   jpNameNormalized,
      //   linkName,
      //   linkNameNormalized,
      //   cardType: card.card_type,
      //   mainColor: cardData['color'],
      //   sideColor: cardData['color2'],
      // });
    } catch (err) {
      console.log(`Failed to update ${card.name} ${card.rarity}`);
      // await Util.sleep(5000);
      // rows.unshift(card);
    } finally {
      await Util.sleep(Math.random() * 100);
    }
  }
}

(async () => {
  await getCardColors();
})();
