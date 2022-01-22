import * as Util from '../../util';

export async function getCardIconUrl(cardId: string): Promise<string | undefined> {
  const filePage = `File:Img${cardId}.png`;
  return Util.WikiPage.getImageUrl(filePage);
}
