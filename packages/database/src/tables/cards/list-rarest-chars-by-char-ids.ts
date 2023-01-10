import { db } from '../../db';
import * as Util from '../../util';
import { CharacterDb, CharacterPublic } from '../characters';
import type { CardDb, CardPublic } from './types';

type CardDbWithMainColor = CardDb & Pick<CharacterDb, 'main_color'>;
type CardWithMainColor = CardPublic & Pick<CharacterPublic, 'mainColor'>;

export async function listRarestCharsByCharIds(charIds: string[]): Promise<CardWithMainColor[]> {
  const dbCards = await db.many<CardDbWithMainColor>(
    `
    SELECT unique_cards.*, characters.main_color
    FROM (
      SELECT DISTINCT ON (char_id) *
      FROM cards
      WHERE char_id IN ($1:csv) AND card_type = 'character'
      ORDER BY char_id, rarity DESC, rarity_modifier DESC NULLS LAST
    ) as unique_cards
    LEFT JOIN characters
    ON unique_cards.char_id = characters.char_id;
    `,
    [charIds],
  );

  const publicCards = dbCards.map((card) => Util.camelCase<CardWithMainColor>(card));
  return publicCards;
}
