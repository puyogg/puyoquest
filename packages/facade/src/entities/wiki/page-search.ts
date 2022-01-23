import * as leven from 'fastest-levenshtein';
import * as Util from '../../util';

export async function pageSearch(query: string): Promise<string[]> {
  const searchResults = await Util.WikiPage.search(query);

  const titlesWithDistances = searchResults.map((result) => {
    return {
      title: result.title,
      distance: leven.distance(Util.normalizeString(result.title), Util.normalizeString(query)),
    };
  });

  titlesWithDistances.sort((a, b) => a.distance - b.distance);

  const titles = titlesWithDistances.map((result) => result.title);
  return titles;
}
