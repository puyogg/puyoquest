import { Collection } from 'discord.js';
import * as Facade from '@ppq-wiki/facade';

interface Combination {
  combinationName: string;
  startingIndex: number;
  length: number;
}

interface CardCombinations {
  charName: string;
  combinations: string[];
}

function getCombinationData(card: Facade.Cards.WikiCard): CardCombinations {
  const combinations = [
    card.combin1,
    card.combin2,
    card.combin3,
    card.combin4,
    card.combin5,
  ].filter((combin): combin is string => !!combin);

  return {
    charName: card.name,
    combinations,
  };
}

function getCombinationDataList(cardList: Facade.Cards.WikiCard[]): CardCombinations[] {
  return cardList.map((card) => getCombinationData(card));
}

function countGroup(
  cards: CardCombinations[],
  startingIndex: number,
  combinationName: string,
): Combination {
  let index = startingIndex;

  while (index < cards.length) {
    if (cards[index].combinations.includes(combinationName)) {
      index++;
    } else {
      break;
    }
  }

  return {
    combinationName,
    startingIndex,
    length: index - startingIndex,
  };
}

export function cardCombinations(cardList: Facade.Cards.WikiCard[]): Combination[] {
  const combinations = new Collection<string, Combination>();

  // Get each character's combinations in list format
  const combinationsList = getCombinationDataList(cardList);

  // Starting at each character...
  for (let c = 0; c < combinationsList.length; c++) {
    // For each of their combinations,
    // get the length of the group subsequence
    const charData = combinationsList[c];
    for (let g = 0; g < charData.combinations.length; g++) {
      const combinationName = charData.combinations[g];

      const combination = countGroup(combinationsList, c, combinationName);
      if (combinations.has(combinationName)) {
        const oldData = combinations.get(combinationName);
        if (oldData && combination.length > oldData.length) {
          combinations.set(combinationName, combination);
        }
      } else if (combination.length >= 3) {
        combinations.set(combinationName, combination);
      }
    }
  }

  return [...combinations.values()];
}
