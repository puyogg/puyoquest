import { WikiPage } from '../wiki-page';
import { distance as LevenDistance } from 'fastest-levenshtein';

export async function findSeriesPageName(userString: string): Promise<string> {
  const seriesName = !userString.toLowerCase().endsWith('series')
    ? `${userString} Series`
    : userString;
  const categorySeriesName = `Category:PPQ:${seriesName}`;
  const pageNames = await WikiPage.search(categorySeriesName);

  if (pageNames.length === 0) {
    throw new Error(`Searching for ${categorySeriesName} returned no results.`);
  }

  const distances = pageNames.map((result) => LevenDistance(result.title, categorySeriesName));
  const indexMinDistance = distances.indexOf(Math.min(...distances));
  return pageNames[indexMinDistance].title;
}
