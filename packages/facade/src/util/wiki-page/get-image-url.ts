import Axios from 'axios';
import { MEDIAWIKI_API_URL } from '../../constants';

interface ImageInfoResponse {
  query: {
    pages: {
      [key: string]: {
        imageinfo?: Array<{
          url?: string;
        }>;
      };
    };
  };
}

/**
 * Fetches the image URL of the requested Wiki file.
 * @param filePage Wiki file page in format: File:Img[charId/artId].png
 */
export async function getImageUrl(filePage: string): Promise<string | undefined> {
  const res = await Axios.get<ImageInfoResponse>(MEDIAWIKI_API_URL, {
    params: {
      action: 'query',
      format: 'json',
      prop: 'imageinfo',
      titles: filePage,
      iiprop: 'url',
    },
  });

  const firstPageKey = Object.keys(res.data.query.pages)[0];
  const page = res.data.query.pages[firstPageKey];

  if (!page.imageinfo) return;

  const firstImageInfo = page.imageinfo[0];
  return firstImageInfo.url;
}
