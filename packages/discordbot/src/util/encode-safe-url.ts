/**
 * Discord's Markdown parser on mobile doesn't play nicely with nested parantheses, so replace them with their
 * URI versions. encodeURI() can't be used for this because it ignores parentheses.
 * @param str URL
 */
export function encodeSafeUrl(url: string): string {
  return encodeURI(url.replace(/\s/g, '_').replaceAll('(', '%28').replaceAll(')', '%29'));
}
