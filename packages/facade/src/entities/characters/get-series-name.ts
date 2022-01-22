import Axios from 'axios';
import { WIKI_BASE_URL } from '../../constants';
import * as Util from '../../util';

export async function getSeriesLink(params: {
  charId: string;
  linkName: string;
}): Promise<{ seriesName: string; isLore: boolean } | undefined> {
  const { charId, linkName } = params;

  const charCardRes = await Axios.get<string>(
    encodeURI(`${WIKI_BASE_URL}/PPQ:${linkName}?action=raw`),
  );
  const charCardTemplate = charCardRes.data;
  const seriesMatch = charCardTemplate.match(/S\d\d\d/);
  if (seriesMatch) {
    const series = seriesMatch[0];
    const seriesTemplateRes = await Axios.get<string>(
      `${WIKI_BASE_URL}/Template:${series}?action=raw`,
    );
    const seriesTemplate = seriesTemplateRes.data;
    const seriesData = Util.parseTemplate<Record<string, string>>(seriesTemplate);

    const seriesName = seriesData['name'];
    const loreKeys = Object.keys(seriesData).filter((key) => key.includes('lore'));
    for (const loreKey of loreKeys) {
      const templateCharId = seriesData[loreKey];
      if (templateCharId === charId) {
        return {
          seriesName,
          isLore: true,
        };
      }
    }

    return {
      seriesName,
      isLore: false,
    };
  } else {
    return;
  }
}
