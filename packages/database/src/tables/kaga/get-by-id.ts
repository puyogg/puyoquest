import { db } from '../../db';
import * as Util from '../../util';
import type { KagaDb, KagaPublic } from './types';

export async function getById(kagaId: string): Promise<KagaPublic | undefined> {
  const kaga = await db.oneOrNone<KagaDb>(`SELECT * FROM kaga WHERE kaga_id = $1`, [kagaId]);

  if (!kaga) return;

  return Util.camelCase<KagaPublic>(kaga);
}
