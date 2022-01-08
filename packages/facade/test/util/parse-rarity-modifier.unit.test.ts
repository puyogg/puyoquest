import { Util } from '../../src/util';

describe('Facade Util.parseRarityModifier()', () => {
  test('Parses 6-2 from linkName (6S characters)', () => {
    const linkName = 'Santa Ringo/★6-2';
    const rarityModifier = Util.parseRarityModifier(linkName);
    expect(rarityModifier).toBe('6-2');
  });
  test('Parses 6-1 from linkName (6S characters)', () => {
    const linkName = 'Santa Ringo/★6-1';
    const rarityModifier = Util.parseRarityModifier(linkName);
    expect(rarityModifier).toBe('6-1');
  });
  test('Parses 6 from linkName (material cards)', () => {
    const linkName = 'Doppelganger Arle/Materials/★6';
    const rarityModifier = Util.parseRarityModifier(linkName);
    expect(rarityModifier).toBe('6');
  });
  test('Parses other rarities from linkName (normal characters)', () => {
    const linkName = 'Paprisu/Red/★4';
    const rarityModifier = Util.parseRarityModifier(linkName);
    expect(rarityModifier).toBe('4');
  });
});
