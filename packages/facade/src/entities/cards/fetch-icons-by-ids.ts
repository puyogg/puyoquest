import * as pMap from 'p-map';
import * as Util from '../../util';

const cachedPage = new Map<string, string>();

export async function fetchIconsByIds(cardIds: string[]): Promise<Buffer[]> {
  const filePages = cardIds.map((cardId) => `File:Img${cardId}.png`);

  const images = await pMap(
    filePages,
    async (filePage) => {
      const cachedUrl = cachedPage.get(filePage);

      if (cachedUrl) {
        return Util.ImageCache.get(cachedUrl);
      } else {
        const imageUrl = await Util.WikiPage.getImageUrl(filePage);

        if (!imageUrl) return;

        cachedPage.set(filePage, imageUrl);
        return Util.ImageCache.get(imageUrl);
      }
    },
    { concurrency: 5 },
  );

  const validImages = images.filter((image): image is Buffer => image !== undefined);
  return validImages;
}
