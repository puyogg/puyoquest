/**
 * Migrate aliases from Yotarou v1
 */

import * as Fs from 'fs';
import * as Path from 'path';
import { db } from '../db';
import { Database } from '..';
import { AliasDb } from '../tables/aliases/types';

interface AliasData {
  fullName: string;
  aliases: string[];
}

const aliasJson: AliasData[] = JSON.parse(
  Fs.readFileSync(Path.join('/srv/packages/database/src/scripts/aliaslist.json'), {
    encoding: 'utf-8',
  }),
);

function normalizeString(str: string, dropS = true): string {
  const strDropS = dropS ? str.trim().replace(/\sS$/, '').replace(/・S$/, '') : str.trim();

  return strDropS
    .replace('☆', ' ')
    .replace(/\/★.*/, '')
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/\s\s+/g, ' ')
    .replace(/\s/g, ' ')
    .toLowerCase();
}

(async () => {
  for (const data of aliasJson) {
    try {
      const nameNormalized = normalizeString(data.fullName);
      const currentData = await db.one<AliasDb>(`SELECT * FROM aliases WHERE alias = $1`, [
        nameNormalized,
      ]);
      await Promise.all(
        data.aliases.map(async (alias) => {
          return Database.Aliases.upsert({
            alias,
            charId: currentData.char_id,
            internal: false,
            cardType: 'character',
          });
        }),
      );
    } catch (err) {
      console.error(data.fullName, err);
    }
  }
})();
