import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

interface CardQuery {
  type: 'card';
  wikiCard: Facade.Cards.WikiCard;
}

interface CharacterQuery {
  type: 'character';
  characterData: Awaited<ReturnType<typeof Facade.Characters.getByName>>;
}

export async function resolveCharacterRarityQuery(
  query: string,
): Promise<CardQuery | CharacterQuery> {
  const parsedQuery = Util.parseCharAndRarityQuery(query);

  if (parsedQuery.query) {
    try {
      const { name, rarity } = parsedQuery.query;

      const wikiCard = await Facade.Cards.getByNameAndRarity({ name, rarity });
      const cardQuery: CardQuery = {
        type: 'card',
        wikiCard,
      };
      return cardQuery;
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
