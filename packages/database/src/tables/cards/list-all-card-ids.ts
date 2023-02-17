import { CardDb } from '.';
import { db } from '../../db';

async function listByIncludeCharIds(charIds: string[]): Promise<string[]> {
  const cards = await db.many<Pick<CardDb, 'card_id'>>(
    'SELECT card_id FROM cards WHERE char_id IN ($1:csv)',
    [charIds],
  );
  return cards.map((card) => card.card_id);
}

async function listByExcludeCharIds(charIds: string[]): Promise<string[]> {
  const cards = await db.many<Pick<CardDb, 'card_id'>>(
    'SELECT card_id FROM cards WHERE NOT char_id IN ($1:csv)',
    [charIds],
  );
  return cards.map((card) => card.card_id);
}

async function listByIncludeExcludeCharIds(params: {
  includeCharIds: string[];
  excludeCharIds: string[];
}): Promise<string[]> {
  const { includeCharIds, excludeCharIds } = params;

  const cards = await db.many<Pick<CardDb, 'card_id'>>(
    `
    SELECT card_id FROM cards
    WHERE char_id IN ($1:csv) AND NOT char_id IN ($2:csv)
    `,
    [includeCharIds, excludeCharIds],
  );

  return cards.map((card) => card.card_id);
}

export async function listAllCardIds(params: {
  includeCharIds?: string[];
  excludeCharIds?: string[];
}): Promise<string[]> {
  const { includeCharIds, excludeCharIds } = params;

  if (includeCharIds?.length && excludeCharIds?.length) {
    return listByIncludeExcludeCharIds({ includeCharIds, excludeCharIds });
  } else if (includeCharIds?.length) {
    return listByIncludeCharIds(includeCharIds);
  } else if (excludeCharIds?.length) {
    return listByExcludeCharIds(excludeCharIds);
  } else {
    throw new Error('No values passed to includeCharIds or excludeCharIds');
  }
}
