import { db } from '../../db';

export async function upsert(kagaId: string, url: string): Promise<void> {
  await db.result(
    `
    INSERT INTO kaga VALUES ($1, $2)
    ON CONFLICT (kaga_id)
    DO UPDATE SET
      url = EXCLUDED.url
    `,
    [kagaId, url],
  );
}
