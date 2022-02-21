import Axios from 'axios';
import { Logger } from '../logger';
import * as pMap from 'p-map';
import { Util } from '..';
import { WIKI_BASE_URL } from '../constants';
import { Database } from '@ppq-wiki/database';

export class IndexRebuilderRecentChanges {
  private intervalMs: number;

  constructor(intervalMs: number = 1000 * 60 * 10) {
    this.intervalMs = intervalMs;
  }

  public async fetchTemplate(id: string): Promise<string> {
    const templateUrl = `${WIKI_BASE_URL}/Template:${id}?action=raw`;
    const templateRes = await Axios.get<string>(templateUrl);
    Logger.AxiosResponse(templateRes);
    return templateRes.data;
  }

  private async rebuildCharacters(charIds: string[]): Promise<void> {
    const retry: string[] = [];

    await pMap(
      charIds,
      async (charId) => {
        try {
          const template = await this.fetchTemplate(charId);
          const characterData = Util.parseTemplate<Record<string, string>>(template);
          const characterLinkName = characterData['link'] || characterData['name'];

          await Database.Characters.create({
            charId,
            name: characterData['name'],
            linkName: characterLinkName,
            jpName: characterData['jpname'],
            mainColor: characterData['color'],
            sideColor: characterData['color2'],
            type1: characterData['type1'],
            type2: characterData['type2'],
            voiceTrans: characterData['voicetrans'],
          });

          await this.rebuildCharacterCards(charId, characterData);
        } catch (err) {
          if (!Axios.isAxiosError(err)) {
            console.error(err);
            return;
          }

          console.error({
            name: err.name,
            code: err.code,
            message: err.message,
            url: err.config.url,
          });

          // Material cards can't be related to the base character from just the 4 digit ID
          if (err.response?.status !== 404) {
            retry.push(charId);
          }
        }
      },
      { concurrency: 2 },
    );

    if (retry.length) {
      setTimeout(() => this.rebuildCharacters(retry), 1000 * 10);
    } else {
      await Database.Cron.setLastWikiRecentChanges();
    }
  }

  private async rebuildCharacterCards(
    charId: string,
    characterData: Record<string, string>,
  ): Promise<void> {
    const cardKeys = Object.keys(characterData).filter((key) => /card(\d+)|mat(\d+)/.test(key));
    const cardIds = cardKeys.map((key) => characterData[key]);

    await Promise.all(
      cardIds.map(async (cardId, i) => {
        const cardKey = cardKeys[i];

        const template = await this.fetchTemplate(cardId);
        const cardData = Util.parseTemplate<Record<string, string>>(template);

        const cardType = /card/i.test(cardKey) ? 'character' : 'material';
        const nameNormalized = Util.normalizeString(cardData['name']);
        const linkName = cardData['link'] || cardData['name'];
        const linkNameNormalized = Util.normalizeString(linkName);
        const rarityModifier = Util.parseRarityModifier(linkName);
        const jpNameNormalized = Util.normalizeString(cardData['jpname']);

        await Database.Aliases.upsert({
          alias: nameNormalized,
          charId,
          internal: true,
          cardType,
        });

        await Database.Aliases.upsert({
          alias: linkNameNormalized,
          charId,
          internal: true,
          cardType,
        });

        if (jpNameNormalized) {
          await Database.Aliases.upsert({
            alias: jpNameNormalized,
            charId,
            internal: true,
            cardType,
          });
        }

        await Database.Cards.upsert({
          cardId: cardData['code'],
          charId,
          rarity: cardData['rarity'],
          rarityModifier,
          name: cardData['name'],
          nameNormalized,
          jpName: cardData['jpname'],
          jpNameNormalized,
          linkName,
          linkNameNormalized,
          cardType,
        });
      }),
    );
  }

  public async start(): Promise<void> {
    try {
      const charIds = await Util.WikiPage.getRecentCharIdChanges();
      await this.rebuildCharacters(charIds);
    } finally {
      setInterval(async () => {
        const charIds = await Util.WikiPage.getRecentCharIdChanges();
        await this.rebuildCharacters(charIds);
      }, this.intervalMs);
    }
  }
}
