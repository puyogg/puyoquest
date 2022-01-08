import { db } from '../../db';
import type { CharacterCreate, CharacterDb } from './types';

export async function create(params: CharacterCreate): Promise<number> {
  const insert: CharacterDb = {
    char_id: params.charId,
    name: params.name,
    link_name: params.linkName,
    jp_name: params.jpName || null,
    main_color: params.mainColor,
    side_color: params.sideColor || null,
    type1: params.type1 || null,
    type2: params.type2 || null,
    voice_trans: params.voiceTrans || null,
    updated_at: new Date(),
  };

  const result = await db.result(
    `
    INSERT INTO characters($1:name)
    VALUES ($1:csv)
    ON CONFLICT (char_id)
    DO UPDATE SET
      name = EXCLUDED.name,
      link_name = EXCLUDED.link_name,
      jp_name = EXCLUDED.jp_name,
      main_color = EXCLUDED.main_color,
      side_color = EXCLUDED.side_color,
      type1 = EXCLUDED.type1,
      type2 = EXCLUDED.type2,
      voice_trans = EXCLUDED.voice_trans,
      updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );

  return result.rowCount;
}
