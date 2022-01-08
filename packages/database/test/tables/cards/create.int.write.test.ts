import * as Db from '../../../src/db';
import { Database } from '../../../src';
import type { CardCreate } from '../../../src/tables/cards';

describe('Database.Cards.create()', () => {
  beforeEach(async () => {
    await Db.createTables();
  });

  afterEach(async () => {
    await Db.dropTables();
  });

  const card: CardCreate = {
    cardId: '420307',
    charId: '4203',
    rarity: '7',
    name: 'Legamünt',
    nameNormalized: 'legamunt',
    jpName: 'レガムント',
    linkName: 'Legamünt',
    cardType: 'character',
  };

  describe('When the card does not exist yet', () => {
    let rowCount: number;

    beforeAll(async () => {
      rowCount = await Database.Cards.create(card);
    });
    it('creates the card without errors', () => {
      expect(rowCount).toBe(1);
    });
  });

  describe('When the card already exists', () => {
    let rowCount: number;

    beforeAll(async () => {
      await Database.Cards.create(card);
      rowCount = await Database.Cards.create({
        ...card,
        name: 'Rozatte',
      });
    });
    it('upserts the card without errors', () => {
      expect(rowCount).toBe(1);
    });
  });
});
