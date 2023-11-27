import { db } from '../../db';
import * as Util from '../../util';
import type { KagaDb, KagaPublic } from './types';

export async function list(): Promise<KagaPublic[]> {
  const allKaga = await db.manyOrNone<KagaDb>('SELECT * FROM kaga');

  return allKaga.map((k) => Util.camelCase<KagaPublic>(k));
}
