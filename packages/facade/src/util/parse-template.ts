function isOpener(str: string): boolean {
  return /(({{{)|({{))/.test(str);
}

function isCloser(str: string): boolean {
  return /((}}})|(}}))/.test(str);
}

export function getCurlyIndexPairs(template: string): [number, number][] {
  const curlyBracketMatches = [...template.matchAll(/({{)|(}})/g)];

  if (curlyBracketMatches.length % 2 !== 0) {
    throw new Error('Unable to parse template. Curly brackets are unbalanced.');
  }

  const indexPairs: [number, number][] = [];
  const stack: RegExpMatchArray[][] = [];

  curlyBracketMatches.forEach((curlyBracketMatch) => {
    const index = curlyBracketMatch.index;
    if (index === undefined) return;

    if (isOpener(curlyBracketMatch[0])) {
      stack.push([curlyBracketMatch]);
    } else if (isCloser(curlyBracketMatch[0])) {
      const [opener] = stack.pop() || [];
      if (opener.index && curlyBracketMatch.index) {
        indexPairs.push([opener.index, curlyBracketMatch.index]);
      }
    }
  });

  return indexPairs;
}

export function parseTemplate(template: string): Record<string, string> {
  // Remove leading {{ and trailing }}
  // Reduce inner triples {{{}}} to doubles {{}}
  // Remove comments? <!--(.+?)-->
  const cleanTemplate = template
    .trim()
    .replace(/(^{{)|(}}$)/g, '')
    .replace(/(?<!\})\}{3}(?!\})/g, '}}')
    .replace(/(?<!\{)\{{3}(?!\{)/g, '{{')
    .replace(/<!--(.+?)-->/g, '');
  const curlyIndexPairs = getCurlyIndexPairs(cleanTemplate);

  // Keep keys that are NOT in-between curly brackets
  const keyMatches = [...cleanTemplate.matchAll(/(\|)(\w+?)(=)/g)];
  const validKeys = keyMatches.filter((keyMatch) => {
    const { index } = keyMatch;
    if (index === undefined) return;

    const indexPair = curlyIndexPairs.find((pair) => {
      return pair[0] < index && index < pair[1];
    });

    return !indexPair;
  });

  // Values are in-between validKeys (or the end of the template)
  const values: string[] = [];
  validKeys.forEach((validKey, i) => {
    let targetIndex: number;
    if (i === validKeys.length - 1) {
      targetIndex = cleanTemplate.length;
    } else {
      targetIndex = validKeys[i + 1].index as number;
    }

    const startIndex = validKey.index as number;
    const value = cleanTemplate.slice(startIndex + validKey[0].length, targetIndex).trim();
    values.push(value);
  });

  const keyValueMap = keyMatches.reduce((acc, key, i) => {
    const cleanKey = key[2];
    const value = values[i];
    acc[cleanKey] = value;
    return acc;
  }, {} as Record<string, string>);

  return keyValueMap;
}
