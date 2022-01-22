import * as Util from '../../util';
import type { SearchResult } from '../../util/wiki-page/search';

export async function pageSearch(query: string): Promise<SearchResult[]> {
  return Util.WikiPage.search(query);
}
