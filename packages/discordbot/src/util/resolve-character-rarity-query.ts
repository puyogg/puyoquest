import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export interface CardQuery {
  type: 'card';
  wikiCard: Facade.Cards.WikiCard;
}

export interface LoreQuery {
  type: 'lore';
  wikiLore: Facade.Cards.WikiLore;
}

export interface CharacterQuery {
  type: 'character';
  characterData: Awaited<ReturnType<typeof Facade.Characters.getByName>>;
}

export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'lore';
}): Promise<LoreQuery | CharacterQuery>;
export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'card';
}): Promise<CardQuery | CharacterQuery>;
export async function resolveCharacterRarityQuery(params: {
  query: string;
  desiredType: 'card' | 'lore';
}): Promise<CardQuery | LoreQuery | CharacterQuery> {
  const { query, desiredType } = params;

  const parsedQuery = Util.parseCharAndRarityQuery(query);

  if (parsedQuery.query) {
    try {
      const { name, rarity } = parsedQuery.query;

      if (desiredType === 'card') {
        const wikiCard = await Facade.Cards.getByNameAndRarity({ name, rarity });
        const cardQuery: CardQuery = {
          type: 'card',
          wikiCard,
        };
        return cardQuery;
      } else {
        const wikiLore = await Facade.Cards.getLoreByNameAndRarity({ name, rarity });
        const loreQuery: LoreQuery = {
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
        const characterQuery: CharacterQuery = {
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
      const characterQuery: CharacterQuery = {
        type: 'character',
        characterData,
      };
      return characterQuery;
    } catch (charLookupError) {
      throw charLookupError;
    }
  }
}
