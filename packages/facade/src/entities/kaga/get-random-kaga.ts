import { Database } from '@ppq-wiki/database';

export async function getRandomKaga(): Promise<Database.Kaga.KagaPublic | undefined> {
  const allKaga = await Database.Kaga.list();
  const randomIdx = Math.floor(Math.random() * allKaga.length);
  return allKaga[randomIdx];
}
