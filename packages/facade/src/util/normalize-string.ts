/**
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/normalize
 * Decomposes canonical text, removes the diacritics, and cleans up other symbols
 * and whitespace. Don't use this on Japanese text because NFD decomposition will make
 * equality comparisons difficult to debug.
 */
export function normalizeString(str: string): string {
  return str
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace('☆', ' ')
    .replace(/\s\s+/g, ' ')
    .replace(/\s/g, ' ')
    .trim()
    .toLowerCase();
}
