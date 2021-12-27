import Axios from 'axios';
import { WIKI_BASE_URL } from '../../constants';
import { Logger } from '../../logger';

export async function getCharIdsFromSeriesId(seriesId: string): Promise<string[]> {
  const url = `${WIKI_BASE_URL}/Template:${seriesId}`;

  const res = await Axios.get<string>(url, {
    params: {
      action: 'raw',
    },
  });
  Logger.AxiosResponse(res);

  const template = res.data;
  const lines = template.split('\n');
  const charLines = lines.filter((line) => line.startsWith('|char') || line.startsWith('|lore'));
  const charIds = charLines.map((charLine) => charLine.split('=')[1]);
  return charIds;
}
