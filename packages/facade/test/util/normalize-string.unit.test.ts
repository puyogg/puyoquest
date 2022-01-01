import { Util } from '../../src/util';

describe('Facade Util.normalizeString()', () => {
  test('Lowercases English latin text', () => {
    const result = Util.normalizeString('MEIKO');
    expect(result).toBe('meiko');
  });
  test('Lowercases and removes diacritics', () => {
    const result = Util.normalizeString('Saucy Legamünt');
    expect(result).toBe('saucy legamunt');
  });
  test(`Replaces Ecolo's star with a space`, () => {
    const result = Util.normalizeString('Space☆Ecolo');
    expect(result).toBe('space ecolo');
  });
  test(`Removes rarity stars`, () => {
    const result = Util.normalizeString('Paprisu/Red/★4');
    expect(result).toBe('paprisu/red/4');
  });
  test('Trims leading or trailing spaces; collapse multiple spaces to one', () => {
    const result = Util.normalizeString('   Space    Ecolo   ');
    expect(result).toBe('space ecolo');
  });
  test('Converts full width spaces to half-width', () => {
    const result = Util.normalizeString('Puyo Puyo　Tetris 2');
    expect(result).toEqual('puyo puyo tetris 2');
  });
  test(`NFD normalization does weird things to Japanese text`, () => {
    const text = 'ぷよぷよフィーバー';
    const result = Util.normalizeString('ぷよぷよフィーバー');
    expect(result).not.toBe(text);
  });
});
