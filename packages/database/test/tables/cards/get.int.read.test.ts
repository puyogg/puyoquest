import { Database } from '../../../src';
import type { CardPublic } from '../../../src/tables/cards';
import * as pgPromise from 'pg-promise';

const QueryResultError = pgPromise.errors.QueryResultError;

describe('Database.Cards.get()', () => {
  it('gets the card if the card exists', async () => {
    const card = await Database.Cards.get('420307');
    expect(card).toEqual<CardPublic>({
      cardId: '420307',
      charId: '4203',
      rarity: '7',
      name: 'Legamünt',
      nameNormalized: 'legamunt',
      jpName: 'レガムント',
      linkName: 'Legamünt',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });

  it('throws an error if the card does not exist', async () => {
    const cardPromise = Database.Cards.get('puyopopfever');
    await expect(cardPromise).rejects.toThrow(QueryResultError);
  });
});
