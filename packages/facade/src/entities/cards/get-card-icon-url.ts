import { Util } from '../..';

export async function getCardIconUrl(cardId: string): Promise<string | undefined> {
  const filePage = `File:Img${cardId}.png`;
  return Util.WikiPage.getImageUrl(filePage);
}
