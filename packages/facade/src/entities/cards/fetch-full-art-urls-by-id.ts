import { WikiPage } from '../../util/wiki-page';

const cachedPage = new Map<string, string>();

export interface FullArtUrls {
  left?: string;
  right?: string;
  special?: string;
}

export async function fetchFullArtUrlsById(cardId: string): Promise<FullArtUrls> {
  const filePages: Record<keyof FullArtUrls, string> = {
    left: `File:Img${cardId}_l.png`,
    right: `File:Img${cardId}_r.png`,
    special: `File:Img${cardId}_ss.png`,
  };

  const resolvedUrls: FullArtUrls = {};
  await Promise.all(
    (Object.entries(filePages) as [keyof FullArtUrls, string][]).map(
      async ([orientation, filePage]) => {
        const cachedUrl = cachedPage.get(filePage);

        if (cachedUrl) {
          resolvedUrls[orientation] = cachedUrl;
        } else {
          let imageUrl: string | undefined;
          try {
            imageUrl = await WikiPage.getImageUrl(filePage);
          } finally {
            if (!imageUrl) return;

            cachedPage.set(filePage, imageUrl);
            resolvedUrls[orientation] = imageUrl;
          }
        }
      },
    ),
  );

  const { left, right, special } = resolvedUrls;
  return {
    ...(left && { left }),
    ...(right && { right }),
    ...(special && { special }),
  };
}
