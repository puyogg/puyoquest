import Axios from 'axios';
import { Logger } from '../logger';
import { WIKI_BASE_URL } from '../constants';
import * as Util from '../util';
import { Database } from '@ppq-wiki/database';
import * as _ from 'lodash';

export class IndexRebuilder {
  private pages: string[];
  private intervalMs: number;
  private linkNames: string[];

  constructor(params?: { pages?: string[]; intervalMs?: number }) {
    this.pages = params?.pages || [
      'Category:PPQ:Red_Color',
      'Category:PPQ:Blue_Color',
      'Category:PPQ:Green_Color',
      'Category:PPQ:Yellow_Color',
      'Category:PPQ:Purple_Color',
    ];

    this.intervalMs = params?.intervalMs || 10000;
    this.linkNames = [];
  }

  public async initLinkNames(): Promise<void> {
    const linkNameGroups = await Promise.all(
      this.pages.map(async (page) => {
        return Util.WikiPage.getAllCategoryLinkNames(page, false);
      }),
    );

    const linkNames = [
      ...new Set(
        linkNameGroups.flat().filter(
          (linkName) =>
            // Ignore Category:PPQ:Red_Color/Cards subcategory
            !/\/Cards$/.test(linkName),
        ),
      ),
    ];

    this.linkNames = _.shuffle(linkNames);
  }

  public async fetchTemplate(id: string): Promise<string> {
    const templateUrl = `${WIKI_BASE_URL}/Template:${id}?action=raw`;
    const templateRes = await Axios.get<string>(templateUrl);
    // Logger.AxiosResponse(templateRes);
    return templateRes.data;
  }

  public async rebuildCharacter(): Promise<void> {
    if (this.linkNames.length === 0) {
      await this.initLinkNames();
    }

    const linkName = this.linkNames.pop();

    const charPageUrl = `${WIKI_BASE_URL}/${linkName}?action=raw`;
    const charPageRes = await Axios.get<string>(charPageUrl);
    Logger.AxiosResponse(charPageRes);

    const charPageRaw = charPageRes.data;
    // Ex. {{184442L|long}} or {{2012|long}}
    // Capture the character id string.
    const charIdMatch = charPageRaw.match(/(\{\{)(\w+?)\|/);
    if (!charIdMatch) return;
    const charId = charIdMatch[2].trim();

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

    setTimeout(() => {
      this.rebuildCharacter().catch((err) => {
        console.error(err);
      });
    }, this.intervalMs + Math.random() * 5000);
  }

  public async rebuildCharacterCards(
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

  public start(): void {
    this.initLinkNames();
    setTimeout(() => {
      this.rebuildCharacter().catch((err) => console.error(err));
    }, this.intervalMs);
  }
}
