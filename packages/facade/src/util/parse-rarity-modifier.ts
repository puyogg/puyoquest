export function parseRarityModifier(linkName: string): string | undefined {
  const modifierMatch = linkName.match(/★\d.*$/);

  let rarityModifier;
  if (modifierMatch) {
    rarityModifier = modifierMatch[0].replace('★', '');
  }

  return rarityModifier;
}
