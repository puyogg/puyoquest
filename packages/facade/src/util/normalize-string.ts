/**
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/normalize
 * Decomposes canonical text, removes the diacritics, and cleans up other symbols
 * and whitespace. Use on character names.
 */
export function normalizeString(str: string, dropS = true): string {
  const strDropS = dropS ? str.trim().replace(/\sS$/, '').replace(/・S$/, '') : str.trim();

  return strDropS
    .replace('☆', ' ')
    .replace(/\/★.*/, '')
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/\s\s+/g, ' ')
    .replace(/\s/g, ' ')
    .toLowerCase();
}
