interface BracketToken {
  type: string;
  index: number;
}

interface BracketPair {
  start: number;
  end: number;
  depth: number;
}

function bracketToken(match: RegExpMatchArray): BracketToken {
  return {
    type: match[0],
    index: match.index as number,
  };
}

function findBracketPairs(template: string) {
  const openers = [...template.matchAll(/{/g)];
  const closers = [...template.matchAll(/}/g)];

  if (openers.length !== closers.length) {
    throw Error('Unable to parse template. Curly brackets are unbalanced.');
  }

  const brackets = [...template.matchAll(/{|}/g)];

  let depth = 0;
  const bracketPairs: BracketPair[] = [];
  const stack: BracketToken[] = [];

  brackets.forEach((match) => {
    const currToken = bracketToken(match);
    if (currToken.type === '{') {
      stack.push(currToken);
      depth += 1;
    } else if (currToken.type === '}') {
      const leftCurly = stack.pop() as BracketToken;
      depth -= 1;
      bracketPairs.push({
        start: leftCurly.index,
        end: currToken.index,
        depth,
      });
    }
  });
  return bracketPairs;
}

export function parseTemplate(template: string) {
  const templateNoComments = template.replace(/<!--[\S\s]+-->/g, '');

  const bracketPairs = findBracketPairs(templateNoComments);
  const ignoreIndexPairs = bracketPairs.filter((pair) => pair.depth >= 2);

  const keyMatches = [...templateNoComments.matchAll(/(\|)(\w+?)(=)/g)];
  const validKeys = keyMatches.filter((keyMatch) => {
    const { index } = keyMatch;
    if (index === undefined) return;

    const indexPair = ignoreIndexPairs.find((pair) => {
      return pair.start < index && index < pair.end;
    });

    return !indexPair;
  });

  // Values are in-between validKeys (or the end of the template)
  const values: string[] = [];
  validKeys.forEach((validKey, i) => {
    let targetIndex: number;
    if (i === validKeys.length - 1) {
      targetIndex = templateNoComments.length - 2; // '}}'
    } else {
      targetIndex = validKeys[i + 1].index as number;
    }

    const startIndex = validKey.index as number;
    const value = templateNoComments.slice(startIndex + validKey[0].length, targetIndex).trim();
    values.push(value);
  });

  const keyValueMap = validKeys.reduce((acc, key, i) => {
    const cleanKey = key[2];
    const value = values[i];
    acc[cleanKey] = value;
    return acc;
  }, {} as Record<string, string>);

  return keyValueMap;
}
