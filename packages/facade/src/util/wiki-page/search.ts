import Axios from 'axios';
import { MEDIAWIKI_API_URL } from '../../constants';
import { Logger } from '../../logger';

export interface SearchResult {
  ns: number;
  timestamp: string;
  title: string;
  pageid: string;
}

export interface SearchResponse {
  batchcomplete: string;
  query: {
    search: SearchResult[];
  };
}

export async function search(str: string): Promise<SearchResult[]> {
  const res = await Axios.get<SearchResponse>(MEDIAWIKI_API_URL, {
    params: {
      action: 'query',
      format: 'json',
      list: 'search',
      srsearch: str,
      srlimit: '500',
      srwhat: 'title',
      srinfo: '',
      srprop: 'timestamp',
    },
  });
  Logger.AxiosResponse(res);

  return res.data.query.search;
}
