import * as Db from '../../../src/db';
import { Database } from '../../../src';
import { LIST_BY_CHAR_ID_QUERIES } from '../../../src/tables/cards';

jest.mock('../../../src/db', () => {
  return {
    db: {
      many: () => [{ card_id: '242407' }],
    },
  };
});

describe('Database.Cards.listByCharId()', () => {
  describe('calls db.many with correct query', () => {
    afterEach(() => {
      jest.clearAllMocks();
    });

    const dbManySpy = jest.spyOn(Db.db, 'many');

    test('includeMaterials = false', async () => {
      const charId = '2424';
      await Database.Cards.listByCharId({ charId });

      expect(dbManySpy.mock.calls[0][0]).toBe(LIST_BY_CHAR_ID_QUERIES.CHARACTERS_ONLY);
      expect(dbManySpy.mock.calls[0][1]).toEqual<string[]>([charId]);
    });
    test('includeMaterials = true', async () => {
      const charId = '2424';
      await Database.Cards.listByCharId({ charId, includeMaterials: true });

      expect(dbManySpy.mock.calls[0][0]).toBe(LIST_BY_CHAR_ID_QUERIES.INCLUDE_MATERIALS);
      expect(dbManySpy.mock.calls[0][1]).toEqual<string[]>([charId]);
    });
  });
});
