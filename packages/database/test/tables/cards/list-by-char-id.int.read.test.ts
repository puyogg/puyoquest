import { Database } from '../../../src';
import * as pgPromise from 'pg-promise';
const QueryResultError = pgPromise.errors.QueryResultError;

describe('Database.Cards.listByCharId()', () => {
  describe('Normal characters (e.g. Santa Ringo)', () => {
    const charId = '3212';
    const charCardIds = ['321204', '321205', '321206', '321216'];
    const matCardIds = ['352905', '352906', '152806'];

    it('gets the complete list of cards', async () => {
      const cards = await Database.Cards.listByCharId({ charId });

      const receivedCardIds = cards.map((card) => card.cardId).sort();
      const expectedCardIds = [...charCardIds].sort();

      expect(receivedCardIds).toEqual<string[]>(expectedCardIds);
    });
    it('gets the complete list of cards including materials', async () => {
      const cards = await Database.Cards.listByCharId({ charId, includeMaterials: true });

      const receivedCardIds = cards.map((card) => card.cardId).sort();
      const expectedCardIds = [...charCardIds, ...matCardIds].sort();

      expect(receivedCardIds).toEqual<string[]>(expectedCardIds);
    });
  });
  describe('Character whose main color changes between rarities (e.g. Gemini Saga)', () => {
    const charId = '4362';
    const charCardIds = ['436205', '436206', '536206'];
    const matCardIds = ['555906'];

    it('gets the complete list of cards', async () => {
      const cards = await Database.Cards.listByCharId({ charId });

      const receivedCardIds = cards.map((card) => card.cardId).sort();
      const expectedCardIds = [...charCardIds].sort();

      expect(receivedCardIds).toEqual<string[]>(expectedCardIds);
    });
    it('gets the complete list of cards including materials', async () => {
      const cards = await Database.Cards.listByCharId({ charId, includeMaterials: true });

      const receivedCardIds = cards.map((card) => card.cardId).sort();
      const expectedCardIds = [...charCardIds, ...matCardIds].sort();

      expect(receivedCardIds).toEqual<string[]>(expectedCardIds);
    });
  });
  describe('charId does not exist', () => {
    it('throws a QueryResultError error', async () => {
      const listPromise = Database.Cards.listByCharId({ charId: 'puyopopfever' });
      await expect(listPromise).rejects.toThrow(QueryResultError);
    });
  });
});
