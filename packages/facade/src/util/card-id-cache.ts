import { Database } from '@ppq-wiki/database';
import { DateTime } from 'luxon';

function diffMs(before: DateTime): number {
  const now = DateTime.utc();
  const diff = now.diff(before);
  return diff.milliseconds;
}

export class CardIdCache {
  private static cardIds: string[] = [];
  private static lastFetched: DateTime = DateTime.utc(1970);
  private static expirationTime = 60 * 1000 * 10; // 10 minutes

  private static async fetch() {
    if (diffMs(CardIdCache.lastFetched) >= CardIdCache.expirationTime) {
      console.log('Fetching card id list');
      CardIdCache.cardIds = await Database.Cards.listAllCardIds({
        excludeCharIds: ['1239', '2239', '3239', '4239', '5239', '1238'],
      });

      CardIdCache.lastFetched = DateTime.utc();
    }
  }

  private static getRandomCardId(): string {
    const cardCount = CardIdCache.cardIds.length;
    const randomIndex = Math.floor(Math.random() * cardCount);
    return CardIdCache.cardIds[randomIndex];
  }

  public static async getRandomCardIds(count: number): Promise<string[]> {
    await CardIdCache.fetch();

    const cardIds: string[] = [];
    for (let i = 0; i < count; i++) {
      const cardId = CardIdCache.getRandomCardId();
      cardIds.push(cardId);
    }

    return cardIds;
  }
}
