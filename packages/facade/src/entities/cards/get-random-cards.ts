import { Database } from '@ppq-wiki/database';
import * as Util from '../../util';

export async function getRandomCards(count: number): Promise<Database.Cards.CardPublic[]> {
  const cardIds = await Util.CardIdCache.getRandomCardIds(count);

  const uniqueCards = await Database.Cards.listById(cardIds);
  const uniqueCardHash = uniqueCards.reduce((acc, card) => {
    acc[card.cardId] = card;
    return acc;
  }, {} as Record<string, Database.Cards.CardPublic>);

  const cards: Database.Cards.CardPublic[] = cardIds
    .map((cardId) => uniqueCardHash[cardId])
    .filter((card) => !!card);

  if (cards.length !== count) {
    throw new Error(`Somehow the random list of cards does not equal the desired count (${count})`);
  }

  return cards;
}
