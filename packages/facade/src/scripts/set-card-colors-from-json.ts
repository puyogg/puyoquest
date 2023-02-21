import { Db } from '@ppq-wiki/database';
import { Database } from '@ppq-wiki/database';
import * as Fs from 'fs';

type CardDb = Database.Cards.CardDb;

async function setCardColors() {
  const cards = JSON.parse(Fs.readFileSync('./card-colors-feb2023.json', { encoding: 'utf-8' }));

  for (const card of cards) {
    const { card_id, main_color, side_color } = card;

    const result = await Db.db.result(
      `
      UPDATE cards
        SET main_color = $2,
            side_color = $3
      WHERE card_id = $1
      `,
      [card_id, main_color, side_color],
    );
    console.log(`Set ${card_id}: ${main_color}, ${side_color}`);
  }
}

(async () => {
  await setCardColors();
})();
