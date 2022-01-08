import * as Facade from '@ppq-wiki/facade';

export function combinationMarkdownLinks(card: Facade.Cards.WikiCard): string {
  const combinations = [
    card.combin1,
    card.combin2,
    card.combin3,
    card.combin4,
    card.combin5,
    card.combin6,
  ].filter((combin): combin is string => !!combin);

  const combinationLinks = combinations.map((combination) => {
    const combinationURI = encodeURI(combination);
    return `[[${combination}]](https://puyonexus.com/wiki/Category:PPQ:${combinationURI}_Combination)`;
  });
  const combinationText = combinationLinks.join(' ');

  return combinationText;
}
