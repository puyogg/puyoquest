import { CardDb, CardPublic } from '.';
import { db } from '../../db';
import * as Util from '../../util';

export async function getByCharIdAndRarity(params: {
  charId: string;
  rarity: string;
  rarityModifier: string | null;
  material?: boolean;
}): Promise<CardPublic> {
  const { charId, rarity, rarityModifier, material = false } = params;

  const cardType = material ? 'material' : 'character';

  if (rarityModifier) {
    const card = await db.one<CardDb>(
      `SELECT * FROM cards WHERE char_id = $1 AND rarity = $2 AND rarity_modifier = $3 AND card_type = $4`,
      [charId, rarity, rarityModifier, cardType],
    );

    return Util.camelCase<CardPublic>(card);
  } else {
    const card = await db.one<CardDb>(
      `
      SELECT * FROM cards
      WHERE
        char_id = $1 AND
        rarity = $2 AND
        (rarity_modifier != '6-2' OR rarity_modifier IS NULL) AND
        card_type = $3
      `,
      [charId, rarity, cardType],
    );

    return Util.camelCase<CardPublic>(card);
  }
}
