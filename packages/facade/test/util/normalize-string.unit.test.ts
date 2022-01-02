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
    expect(result).toBe('paprisu/red');
  });
  test('Trims leading or trailing spaces; collapse multiple spaces to one', () => {
    const result = Util.normalizeString('   Space    Ecolo   ');
    expect(result).toBe('space ecolo');
  });
  test('Converts full width spaces to half-width', () => {
    const result = Util.normalizeString('Puyo Puyo　Tetris 2');
    expect(result).toEqual('puyo puyo tetris 2');
  });
  test('Removes S modifier when dropS = true', () => {
    const result = Util.normalizeString('Santa Ringo S');
    expect(result).toEqual('santa ringo');
  });
  test('Removes S modifier for Japanese names', () => {
    const result = Util.normalizeString('サンタりんご・S');
    expect(result).toEqual('サンタりんご');
  });
});
