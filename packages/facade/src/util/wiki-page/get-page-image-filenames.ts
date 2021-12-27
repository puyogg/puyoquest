import Axios from 'axios';
import { MEDIAWIKI_API_URL } from '../../constants';
import { Logger } from '../../logger';

interface GetPageImageFileNames {
  batchcomplete: string;
  query: {
    pages: {
      [key: string]: {
        images: Array<{
          ns: number;
          title: string;
        }>;
        ns: number;
        pageid: number;
        title: string;
      };
    };
  };
}

export async function getPageImageFilenames(pageTitle: string): Promise<string[]> {
  const res = await Axios.get<GetPageImageFileNames>(MEDIAWIKI_API_URL, {
    params: {
      action: 'query',
      format: 'json',
      prop: 'images',
      titles: pageTitle,
      imlimit: '75',
    },
  });
  Logger.AxiosResponse(res);

  const data = res.data;
  const pages = Object.values(data.query.pages);
  const images = pages.map((page) => page.images.map((image) => image.title)).flat();
  return images;
}
