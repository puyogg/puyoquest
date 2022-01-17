import { Util } from '../../util';
import * as leven from 'fastest-levenshtein';
import { Database } from '@ppq-wiki/database';
import { puyoColorIntMap } from '../../constants';

interface SearchResult {
  pageid: string;
  title: string;
  timestamp: string;
  distance: number;
  accuracy: number;
}

async function searchCategoryPage(categoryPage: string): Promise<SearchResult | undefined> {
  const pageResults = await Util.WikiPage.search(categoryPage);

  if (pageResults.length === 0) {
    return;
  }

  const resultsWithAccuracy = pageResults.map((result) => {
    const { pageid, title, timestamp } = result;

    const distance = leven.distance(categoryPage, result.title);
    const accuracy = 1 - distance / Math.max(title.length, categoryPage.length);

    return { pageid, title, timestamp, distance, accuracy };
  });

  const accuracyPerPage = resultsWithAccuracy.map((result) => result.accuracy);

  return resultsWithAccuracy[accuracyPerPage.indexOf(Math.max(...accuracyPerPage))];
}

async function resolveCategoryNames(
  requestedCategoryNames: string[],
): Promise<{ categoriesFound: SearchResult[]; categoriesNotFound: string[] }> {
  const requestedCategories = requestedCategoryNames.map(
    (categoryName) => `Category:PPQ:${categoryName}`,
  );

  const searchResults = await Promise.all(requestedCategories.map(searchCategoryPage));
  const categoriesFound: SearchResult[] = [];
  const categoriesNotFound: string[] = [];
  searchResults.forEach((result, i) => {
    if (result === undefined) {
      categoriesNotFound.push(requestedCategories[i]);
    } else {
      categoriesFound.push(result);
    }
  });

  return {
    categoriesFound,
    categoriesNotFound,
  };
}

export async function listByCategories(params: {
  requestedCategories: string[];
  mainColor: string | null;
}) {
  const { requestedCategories, mainColor } = params;

  const { categoriesFound, categoriesNotFound } = await resolveCategoryNames(requestedCategories);

  const categoryMemberArrays = await Promise.all(
    categoriesFound.map((result) => Util.WikiPage.getAllCategoryLinkNames(result.title)),
  );

  const categoryMemberSets = categoryMemberArrays.map(
    (categoryMembers) => new Set(categoryMembers),
  );

  const setSizes = categoryMemberSets.map((set) => set.size);
  const largestMemberArray = categoryMemberArrays[setSizes.indexOf(Math.max(...setSizes))];

  const validMembers = largestMemberArray.filter((member) => {
    return categoryMemberSets.every((set) => set.has(member));
  });

  if (validMembers.length === 0) {
    throw Error(`There are no cards that fit all categories: ${categoriesFound.join(', ')}`);
  }

  const validCharIds = await Database.Aliases.list(validMembers);
  validCharIds.sort();

  let cards = await Database.Cards.listRarestCharsByCharIds(validCharIds);

  const normalizedMainColor = mainColor && mainColor.toLowerCase();
  if (
    normalizedMainColor &&
    Object.keys(puyoColorIntMap).some((key) => key === normalizedMainColor)
  ) {
    const colorInt = puyoColorIntMap[normalizedMainColor];
    cards = cards.filter((card) => card.cardId.startsWith(colorInt));
  }

  return {
    cards,
    categoriesFound,
    categoriesNotFound,
  };
}
