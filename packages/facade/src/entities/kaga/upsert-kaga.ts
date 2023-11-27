import { Database } from '@ppq-wiki/database';

export async function upsertKaga(kagaId: string, url: string): Promise<void> {
  return Database.Kaga.upsert(kagaId, url);
}
