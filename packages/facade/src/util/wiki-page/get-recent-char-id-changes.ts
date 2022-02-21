import Axios, { AxiosResponse } from 'axios';
import { Database } from '@ppq-wiki/database';
import { MEDIAWIKI_API_URL } from '../../constants';
import { Logger } from '../../logger';

interface RecentChangesResponse {
  batchcomplete: '';
  query: {
    recentchanges: Array<{
      title: string;
    }>;
  };
  continue?: {
    rccontinue: string;
    continue: string;
  };
}

export async function getRecentCharIdChanges(): Promise<string[]> {
  const lastWikiCheck = await Database.Cron.getLastWikiRecentChanges();
  const now = new Date();

  let hasContinue = true;
  let listContinue;
  let rccontinue;
  const templatePageSet = new Set<string>();

  while (hasContinue) {
    const res: AxiosResponse<RecentChangesResponse> = await Axios.get(MEDIAWIKI_API_URL, {
      params: {
        action: 'query',
        format: 'json',
        list: 'recentchanges',
        rclimit: '500',
        rcstart: lastWikiCheck.toISOString(),
        rcend: now.toISOString(),
        rcdir: 'newer',
        ...(listContinue ? { continue: listContinue } : {}),
        ...(rccontinue ? { rccontinue } : {}),
      },
    });
    Logger.AxiosResponse(res);

    const data = res.data;
    if (data.continue) {
      rccontinue = data.continue.rccontinue;
      listContinue = data.continue.continue;
    } else {
      hasContinue = false;
    }

    data.query.recentchanges.forEach((recentChange) => {
      const { title } = recentChange;

      if (title.startsWith('Template:')) {
        templatePageSet.add(title);
      }
    });
  }

  const templateIds = Array.from(templatePageSet)
    .map((title) => title.split(':')[1])
    .filter((title) => {
      const isDefined = !!title;
      const validCharCardId = /^[0-9]{6}$|^[0-9]{4}$/.test(title);
      return isDefined && validCharCardId;
    })
    .map((id) => id.slice(0, 4));
  const uniqueCharIds = [...new Set(templateIds)];
  return uniqueCharIds;
}
