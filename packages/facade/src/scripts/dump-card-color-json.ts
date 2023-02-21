import { Db } from '@ppq-wiki/database';
import { Database } from '@ppq-wiki/database';
import * as Fs from 'fs';

type CardDb = Database.Cards.CardDb;

async function dumpCardColors() {
  const rows = await Db.db.many<Pick<CardDb, 'card_id' | 'main_color' | 'side_color'>>(
    'SELECT card_id, main_color, side_color FROM cards ORDER BY card_id',
  );

  Fs.writeFileSync('./card-colors-feb2023.json', JSON.stringify(rows, undefined, 2), {
    encoding: 'utf-8',
  });
}

(async () => {
  await dumpCardColors();
})();
