import * as Util from '../../util';

export async function getSeriesIdFromPage(pageTitle: string): Promise<string> {
  const rawText = await Util.WikiPage.getRawText(pageTitle);

  // The series tag will have the form {{s012|long}}
  const seriesIdMatch = rawText.match(/{{(.*?)\|/);
  const id = seriesIdMatch && seriesIdMatch[1];

  if (!id) {
    throw Error(`Failed to parse template: ${rawText}`);
  }

  return id;
}

// if (require.main === module) {
//   (async () => {
//     await getSeriesIdFromPage('Original Puyo Puyo Series');
//   })();
// }
