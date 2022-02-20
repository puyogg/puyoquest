import Axios, { AxiosResponse } from 'axios';
import { Logger } from '../../logger';
import { MEDIAWIKI_API_URL } from '../../constants';
import * as Util from '../../util';

interface CategoryResponse {
  batchcomplete: '';
  query: {
    categorymembers: Array<{
      ns: number;
      pageid: number;
      title: string;
    }>;
  };
  continue?: {
    cmcontinue: string;
    continue: string;
  };
}

export async function getAllCategoryLinkNames(
  categoryPage: string,
  normalize = true,
): Promise<string[]> {
  let hasContinue = true;
  let listContinue;
  let cmcontinue;
  const linkNameSet: Set<string> = new Set();

  while (hasContinue) {
    const res: AxiosResponse<CategoryResponse> = await Axios.get(MEDIAWIKI_API_URL, {
      params: {
        action: 'query',
        format: 'json',
        list: 'categorymembers',
        cmtitle: categoryPage,
        cmlimit: '500',
        ...(listContinue ? { continue: listContinue } : {}),
        ...(cmcontinue ? { cmcontinue } : {}),
      },
    });
    Logger.AxiosResponse(res);

    const data = res.data;
    if (data.continue) {
      cmcontinue = data.continue.cmcontinue;
      listContinue = data.continue.continue;
    } else {
      hasContinue = false;
    }

    const linkNames = data.query.categorymembers.map((member) =>
      normalize ? Util.normalizeString(member.title.replace(/^PPQ:|\/★.+$/g, '')) : member.title,
    );
    linkNames.forEach((linkName) => linkNameSet.add(linkName));
  }

  return [...linkNameSet];
}
