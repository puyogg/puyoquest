import { DbPool } from '../../db-pool';

export interface Character {
  charId: string;
  name: string;
  jpName?: string;
  mainColor: string;
  sideColor?: string;
  type1?: string;
  type2?: string;
  voiceTrans?: string;
}

export async function create(params: Character): Promise<void> {
  const { charId, name, jpName, mainColor, sideColor, type1, type2, voiceTrans } = params;

  const updatedAt = new Date();

  const client = await DbPool.connect();
  try {
    await client.query(
      `
      INSERT INTO characters(char_id, name, jp_name, main_color, side_color, type1, type2, voice_trans, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
      ON CONFLICT (char_id)
      DO UPDATE SET
        name = EXCLUDED.name,
        jp_name = EXCLUDED.jp_name,
        main_color = EXCLUDED.main_color,
        side_color = EXCLUDED.side_color,
        type1 = EXCLUDED.type1,
        type2 = EXCLUDED.type2,
        voice_trans = EXCLUDED.voice_trans,
        updated_at = EXCLUDED.updated_at
    `,
      [charId, name, jpName, mainColor, sideColor, type1, type2, voiceTrans, updatedAt],
    );
  } finally {
    client.release();
  }
}
