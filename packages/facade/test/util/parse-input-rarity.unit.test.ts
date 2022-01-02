import { Util } from '../../src/util';

describe('Facade Util.parseInputRarity()', () => {
  test('Input rarity 6S returns 6, 6-2', () => {
    const inputRarity = '6S';
    const { rarity, rarityModifier } = Util.parseInputRarity(inputRarity);
    expect(rarity).toBe('6');
    expect(rarityModifier).toBe('6-2');
  });
  test('Input rarity 6-1 returns 6, 6-1', () => {
    const inputRarity = '6-1';
    const { rarity, rarityModifier } = Util.parseInputRarity(inputRarity);
    expect(rarity).toBe('6');
    expect(rarityModifier).toBe('6-1');
  });
  test('Input rarity 6 returns 6, null', () => {
    const inputRarity = '6';
    const { rarity, rarityModifier } = Util.parseInputRarity(inputRarity);
    expect(rarity).toBe('6');
    expect(rarityModifier).toBe(null);
  });
  test('Other rarities return rarity, null', () => {
    const inputRarity = '4';
    const { rarity, rarityModifier } = Util.parseInputRarity(inputRarity);
    expect(rarity).toBe('4');
    expect(rarityModifier).toBe(null);
  });
  test('Throws an error for invalid rarities', () => {
    expect(() => Util.parseInputRarity('A')).toThrow('Invalid rarity requested: A');
  });
});
