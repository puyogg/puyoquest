import { Database } from '@ppq-wiki/database';

export async function getById(kagaId: string): Promise<Database.Kaga.KagaPublic | undefined> {
  return Database.Kaga.getById(kagaId);
}
