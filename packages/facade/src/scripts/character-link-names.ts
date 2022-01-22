import { Db } from '@ppq-wiki/database';
import * as Util from '../util';
import { Database } from '@ppq-wiki/database';
import Axios from 'axios';
import { WIKI_BASE_URL } from '../constants';

async function characterLinkNames() {
  const rows = await Db.db.many<Database.Characters.CharacterDb>(`SELECT * FROM characters`);

  for (const row of rows) {
    try {
      const templateRes = await Axios.get<string>(`${WIKI_BASE_URL}/Template:${row.char_id}`);
      const template = templateRes.data;

      const wikiCharacter = Util.parseTemplate<Record<string, string>>(template);

      await Database.Characters.create({
        charId: row.char_id,
        name: row.name,
        linkName: wikiCharacter['link'] || row.name,
        jpName: row.jp_name || undefined,
        mainColor: row.main_color,
        sideColor: row.side_color || undefined,
        type1: row.type1 || undefined,
        type2: row.type2 || undefined,
        voiceTrans: row.voice_trans || undefined,
      });
    } catch (err) {
      console.error(row.name, err);
    }
  }
}

(async () => {
  await characterLinkNames();
})();
