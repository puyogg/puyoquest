export function parseCharAndRarityQuery(query: string): {
  query?: { name: string; rarity: string };
  fallback: { name: string };
} {
  const trimmedQuery = query.trim().replace(/\s\s+/, ' ');

  const match = trimmedQuery.match(/(.+)( )(\d.*$)/);

  if (match) {
    const name = match[1];
    const rarity = match[3];

    return {
      query: { name, rarity },
      fallback: { name: trimmedQuery },
    };
  } else {
    return {
      fallback: { name: trimmedQuery },
    };
  }
}
