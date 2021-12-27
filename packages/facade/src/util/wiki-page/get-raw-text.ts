import Axios from 'axios';
import { WIKI_BASE_URL } from '../../constants';
import { Logger } from '../../logger';

export async function getRawText(pageTitle: string): Promise<string> {
  const url = `${WIKI_BASE_URL}/${pageTitle}`;

  const res = await Axios.get<string>(url, {
    params: {
      action: 'raw',
    },
  });
  Logger.AxiosResponse(res);

  const data = res.data;
  return data;
}
