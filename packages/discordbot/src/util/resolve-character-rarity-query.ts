import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export interface CardWikiResponse {
  type: 'card';
  wikiCard: Facade.Cards.WikiCard;
}

export interface LoreWikiResponse {
  type: 'lore';
  wikiLore: Facade.Cards.WikiLore;
}

export interface CharacterWikiResponse {
  type: 'character';
  characterData: Awaited<ReturnType<typeof Facade.Characters.getByName>>;
}

export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'lore';
  material?: boolean;
}): Promise<LoreWikiResponse | CharacterWikiResponse>;
export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'card';
  material?: boolean;
}): Promise<CardWikiResponse | CharacterWikiResponse>;
export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'card' | 'lore';
  material?: boolean;
}): Promise<CardWikiResponse | LoreWikiResponse | CharacterWikiResponse> {
  const { query, desiredType, material = false } = params;

  const parsedQuery = Util.parseCharAndRarityQuery(query);

  if (parsedQuery.query) {
    try {
      const { name, rarity } = parsedQuery.query;

      if (desiredType === 'card') {
        const wikiCard = await Facade.Cards.getByNameAndRarity({ name, material, rarity });
        const cardQuery: CardWikiResponse = {
          type: 'card',
          wikiCard,
        };
        return cardQuery;
      } else {
        const wikiLore = await Facade.Cards.getLoreByNameAndRarity({ name, material, rarity });
        const loreQuery: LoreWikiResponse = {
          type: 'lore',
          wikiLore,
        };
        return loreQuery;
      }
    } catch (cardLookupError) {
      // Fallback for characters whose actual names end in a number.
      // E.g.:
      // - Kamen Rider 1
      // - Schezo ver. Division 24
      try {
        const { name } = parsedQuery.fallback;
        const characterData = await Facade.Characters.getByName({ name, includeMaterials: true });
        const characterQuery: CharacterWikiResponse = {
          type: 'character',
          characterData,
        };
        return characterQuery;
      } catch (charLookupError) {
        throw charLookupError;
      }
    }
  } else {
    try {
      const { name } = parsedQuery.fallback;
      const characterData = await Facade.Characters.getByName({ name, includeMaterials: true });
      const characterQuery: CharacterWikiResponse = {
        type: 'character',
        characterData,
      };
      return characterQuery;
    } catch (charLookupError) {
      throw charLookupError;
    }
  }
}
