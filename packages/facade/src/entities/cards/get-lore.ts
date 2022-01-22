import Axios from 'axios';
import * as _ from 'lodash';
import * as Util from '../../util';
import { WIKI_BASE_URL } from '../../constants';

export interface WikiLore {
  name: string;
  jpname?: string;
  rarity: string;
  code: string;
  link: string;
  color?: string;
  /** JP Description */
  ft?: string;
  /** English Description */
  fta?: string;
  /** Translator */
  ftc?: string;
  ft1?: string;
  fta1?: string;
  ft2?: string;
  fta2?: string;
  ft3?: string;
  fta3?: string;
}

export async function getLore(cardId: string): Promise<WikiLore> {
  const cardTemplatePageUrl = `${WIKI_BASE_URL}/Template:${cardId}?action=raw`;
  const cardTemplatePageRes = await Axios.get<string>(cardTemplatePageUrl);

  const wikiLore = Util.parseTemplate<WikiLore>(cardTemplatePageRes.data);

  if (!wikiLore.link) {
    wikiLore.link = `${wikiLore.name}/★${wikiLore.rarity}`;
  }

  wikiLore.color = wikiLore.color?.toLowerCase();

  const requiresWikiTextParsing = [
    'ft',
    'fta',
    'ftc',
    'ft1',
    'fta1',
    'ft2',
    'fta2',
    'ft3',
    'fta3',
  ] as const;

  await Promise.all(
    requiresWikiTextParsing.map(async (key) => {
      const value = wikiLore[key];
      if (value) {
        wikiLore[key] = await Util.WikiPage.parseWikiText(value);
      }
    }),
  );

  // clean up undefined keys
  const definedWikiLore = _.pickBy(wikiLore, (v) => v !== undefined) as WikiLore;
  return definedWikiLore;
}
