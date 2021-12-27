// import { PageSeries } from '.';
// import { CardIndex, IndexData } from '../../util/build-index';
// // import { WikiPage } from '../wiki-page';

// export async function getSeriesCharacters(params: {
//   categorySeriesName: `Category:PPQ:${string}`;
//   cardIndex: CardIndex;
// }): Promise<IndexData[]> {
//   const { categorySeriesName, cardIndex } = params;

//   const seriesId = await PageSeries.getSeriesIdFromPage(categorySeriesName);
//   const charIds = await PageSeries.getCharIdsFromSeriesId(seriesId);

//   // Use 4 digit charId to look up card in the index
//   const charData: IndexData[] = [];
//   charIds.forEach((charId) => {
//     const data = cardIndex.byId.get(charId);
//     if (data) charData.push(data);
//   });

//   return charData;
// }
