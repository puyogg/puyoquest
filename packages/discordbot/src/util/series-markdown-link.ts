import * as Facade from '@ppq-wiki/facade';

export async function seriesMarkdownLink(params: {
  charId: string;
  linkName: string;
}): Promise<string | undefined> {
  const { charId, linkName } = params;

  const seriesData = await Facade.Characters.getSeriesLink({
    charId,
    linkName,
  });

  if (seriesData) {
    const { seriesName, isLore } = seriesData;
    const seriesNameURI = encodeURI(seriesName);
    const seriesText = `[${seriesName}${
      isLore ? ' (Lore)' : ''
    }](https://puyonexus.com/wiki/Category:PPQ:${seriesNameURI})`;

    return seriesText;
  } else {
    return;
  }
}
