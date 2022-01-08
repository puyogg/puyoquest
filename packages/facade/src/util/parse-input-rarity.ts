import * as CONSTANTS from '../constants';

/**
 * Parse rarities received from an external source
 */
export function parseInputRarity(inputRarity: string): {
  rarity: string;
  rarityModifier: string | null;
} {
  const inputNormalized = inputRarity.toLowerCase();

  if (!CONSTANTS.VALID_RARITY_REQUESTS.includes(inputNormalized)) {
    throw Error(`Invalid rarity requested: ${inputRarity}`);
  }

  if (inputNormalized === '6s') {
    return {
      rarity: '6',
      rarityModifier: '6-2',
    };
  } else if (inputNormalized === '6-2' || inputNormalized === '6-1') {
    return {
      rarity: '6',
      rarityModifier: inputNormalized,
    };
  } else {
    return {
      rarity: inputNormalized,
      rarityModifier: null,
    };
  }
}
