import * as Facade from '@ppq-wiki/facade';

export function cardTitleRarityLink(card: Facade.Cards.WikiCard) {
  const displayedRarity = card.link.match(/\/★6-2$/) ? '6S' : card.rarity;
  let title = `${card.name} ★${displayedRarity}`;
  if (card.jpname) title += ` (${card.jpname})`;

  const url = encodeURI(`https://puyonexus.com/wiki/PPQ:${card.link}`);

  return { title, url };
}
