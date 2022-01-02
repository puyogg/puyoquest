import { Db } from '@ppq-wiki/database';
import { Util } from '../util';
import { Database } from '@ppq-wiki/database';

type CardDb = Database.Cards.CardDb;

async function reduceLinkNames() {
  const rows = await Db.db.many<CardDb>('SELECT * FROM cards');

  for (const row of rows) {
    const linkNameNormalized = Util.normalizeString(row.link_name);

    await Database.Aliases.upsert({
      alias: row.name_normalized,
      charId: row.char_id,
      internal: true,
      cardType: row.card_type,
    });

    await Database.Aliases.upsert({
      alias: linkNameNormalized,
      charId: row.char_id,
      internal: true,
      cardType: row.card_type,
    });

    if (row.jp_name_normalized) {
      await Database.Aliases.upsert({
        alias: row.jp_name_normalized,
        charId: row.char_id,
        internal: true,
        cardType: row.card_type,
      });
    }

    await Db.db.result(`UPDATE cards SET link_name_normalized = $1 WHERE card_id = $2`, [
      linkNameNormalized,
      row.card_id,
    ]);
  }
}

(async () => {
  await reduceLinkNames();
})();
