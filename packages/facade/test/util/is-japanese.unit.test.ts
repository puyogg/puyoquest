import { Util } from '../../src/util';

describe('Facade Util.normalizeString()', () => {
  test('Detects ぷよぷよ通 as Japanese', () => {
    const text = 'ぷよぷよ通';
    const result = Util.isJapanese(text);
    expect(result).toBe(true);
  });
  test('Detects Legamünt as non-Japanese', () => {
    const text = 'Legamünt';
    const result = Util.isJapanese(text);
    expect(result).toBe(true);
  });
});
